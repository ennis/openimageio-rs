use std::{ffi::CStr, os::raw::c_char};

mod attribute;
mod buffer;
mod cache;
mod error;
mod input;
mod output;
mod roi;
mod spec;
mod typedesc;

pub use buffer::ImageBuffer;
pub use error::Error;
pub use input::ImageInput;
pub use output::{ImageOutput, MultiImageOutput, SingleImageOutput};
pub use spec::{Channel, ChannelDesc, ImageSpec, ImageSpecOwned};
pub use typedesc::{Aggregate, BaseType, TypeDesc, VecSemantics, ImageData};

pub use cache::ImageCache;

unsafe fn cstring_to_owned(cstr: *const c_char) -> String {
    // assume utf8 input
    let msg = CStr::from_ptr(cstr).to_str().unwrap().to_owned();
    openimageio_sys::OIIO_freeString(cstr);
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_image() {
        let img = ImageInput::open("test_images/tonberry.jpg");
        assert!(img.is_ok());
    }

    /// test all API forms
    #[test]
    fn test_api() {
        let mut img = ImageInput::open("test_images/kazeharu.png").unwrap();

        // members on imageinput
        img.spec();
        img.width();
        img.height();
        img.depth();
        img.read::<f32>().unwrap();
        img.all_channels();
        img.channels(0..1).unwrap();
        img.channels_by_name(&["R"]).unwrap();
        img.rgba_channels().unwrap();
        img.alpha_channel().unwrap();
        img.all_channels().read::<f32>().unwrap();

        // members on subimage
        let sub = img.subimage(0).unwrap();
        sub.spec();
        sub.width();
        sub.height();
        sub.depth();
        sub.read::<f32>().unwrap(); // consumes
        img.subimage(0).unwrap().spec();
        img.subimage(0).unwrap().width();
        img.subimage(0).unwrap().height();
        img.subimage(0).unwrap().depth();
        img.subimage(0).unwrap().all_channels();
        img.subimage(0).unwrap().channels(0..1).unwrap();
        img.subimage(0).unwrap().channels_by_name(&["R"]).unwrap();
        img.subimage(0).unwrap().rgba_channels().unwrap();
        img.subimage(0).unwrap().channel_alpha().unwrap();
        img.subimage(0).unwrap().read::<f32>().unwrap();
        img.subimage(0)
            .unwrap()
            .all_channels()
            .read::<f32>()
            .unwrap();

        // members on subimage (same as above, but through subimage+mipmap)
        let sub = img.subimage_mipmap(0, 0).unwrap();
        sub.spec();
        sub.width();
        sub.height();
        sub.depth();
        sub.read::<f32>().unwrap(); // consumes
        img.subimage_mipmap(0, 0).unwrap().spec();
        img.subimage_mipmap(0, 0).unwrap().width();
        img.subimage_mipmap(0, 0).unwrap().height();
        img.subimage_mipmap(0, 0).unwrap().depth();
        img.subimage_mipmap(0, 0).unwrap().all_channels();
        img.subimage_mipmap(0, 0).unwrap().channels(0..1).unwrap();
        img.subimage_mipmap(0, 0)
            .unwrap()
            .channels_by_name(&["R"])
            .unwrap();
        img.subimage_mipmap(0, 0).unwrap().rgba_channels().unwrap();
        img.subimage_mipmap(0, 0).unwrap().channel_alpha().unwrap();
        img.subimage_mipmap(0, 0).unwrap().read::<f32>().unwrap();
        img.subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .read::<f32>()
            .unwrap();

        // members of SubimageMipmapChannels
        let chan = img.subimage_mipmap(0, 0).unwrap().all_channels();
        chan.spec();
        chan.width();
        chan.height();
        chan.depth();
        chan.read::<f32>().unwrap(); // consumes
        img.subimage_mipmap(0, 0).unwrap().all_channels().spec();
        img.subimage_mipmap(0, 0).unwrap().all_channels().width();
        img.subimage_mipmap(0, 0).unwrap().all_channels().height();
        img.subimage_mipmap(0, 0).unwrap().all_channels().depth();
        img.subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .read::<f32>()
            .unwrap();
    }

    #[test]
    fn read_image_scanline() {
        // scanline based png
        let mut img = ImageInput::open("test_images/kazeharu.png").unwrap();
        assert!(img.spec().tile_width() == 0);

        let mut img_data: Vec<u8> = Vec::new();
        img_data.resize(img.spec().width() as usize * img.spec().num_channels(), 0);

        // reading all scanlines should be the same as reading the whole image
        let mut all_scanlines: Vec<u8> = Vec::new();
        for y in 0..img.spec().height() {
            img.read_scanline(y as i32, 0, &mut img_data).unwrap();
            all_scanlines.extend(img_data.clone().iter());
        }

        let whole_img: ImageBuffer<u8> = img.read().unwrap();
        assert_eq!(all_scanlines, whole_img.data);
    }

    #[test]
    fn write_image_scanline() {
        // scanline based png
        let mut img = ImageInput::open("test_images/kazeharu.png").unwrap();
        let data: ImageBuffer<u8> = img.read().unwrap();

        let mut out = ImageOutput::create("kazeharu.png").unwrap();
        let mut out = out.open(&img.spec()).unwrap();
        out.write_image(data.data()).unwrap();

        let mut out2 = ImageOutput::create("kazeharu_scanline.png").unwrap();
        let mut out2 = out2.open(&img.spec()).unwrap();
        let row_width: usize = data.width * data.num_channels;
        for y in 0..data.height() as usize {
            out2.write_scanline(y as i32, 0, &data.data[(y * row_width)..((y + 1) * row_width)]).unwrap();
        }

        // Written image should be identical
        fn file_contents(f: &str) -> Vec<u8> {
            use std::io::Read;
            use std::fs::File;
            let mut file = File::open(f).unwrap();
            let mut res = Vec::new();
            file.read_to_end(&mut res).unwrap();
            res
        }

        assert_eq!(file_contents("kazeharu.png"), file_contents("kazeharu_scanline.png"));
    }

    #[test]
    fn read_tiled() {
        let mut img = ImageInput::open("test_images/tiled.tif").unwrap();
        assert!(img.spec().tile_width() > 0);
        // read first tile, verify it matches the same data of the whole image read with 'read()'
        let mut tiled_data: Vec<u8> = Vec::new();
        tiled_data.resize(img.spec().tile_width() * img.spec().tile_height() * img.spec().num_channels(), 0);
        img.read_tile(0, 0, 0, &mut tiled_data).unwrap();

        let mut whole_img: ImageBuffer<u8> = img.read().unwrap();

        assert!(whole_img.data().len() > tiled_data.len());
        // Manually split out the specific tile we read from the whole image 
        let tile_from_whole_img: Vec<u8> = 
            whole_img.data
            .chunks_mut(img.spec().width() as usize * img.spec().num_channels()) // image rows
            .map(|chunk| {
                chunk[0..(img.spec().tile_width() * img.spec().num_channels())].to_vec() // chop down to tile width
            })
            .take(img.spec().tile_height()) // chop down to tile height
            .flatten() // re-merge chunks
            .collect();

        assert_eq!(tile_from_whole_img, tiled_data);
    }

    #[test]
    fn open_image_exr() {
        let mut img = ImageInput::open("test_images/output0013.exr").unwrap();
        for c in img.spec().channels() {
            println!("{:?}", c);
        }

        let channel_names = &[
            "RenderLayer.DiffCol.R",
            "RenderLayer.DiffCol.G",
            "RenderLayer.DiffCol.B",
        ];
        let size = (img.width(), img.height());
        let data: ImageBuffer<f32> = img.channels_by_name(channel_names).unwrap().read().unwrap();
        let spec = ImageSpecOwned::new_2d(TypeDesc::FLOAT, size.0, size.1, &["R", "G", "B"]);
        let mut out = ImageOutput::create("output.exr").unwrap();
        let mut out = out.open(&spec).unwrap();
        out.write_image(data.data()).unwrap();
    }

    #[test]
    fn open_image_png() {
        let mut img = ImageInput::open("test_images/kazeharu.png").unwrap();
        let size = (img.width(), img.height());
        let data = img.rgba_channels().unwrap().read::<f32>().unwrap();
        let spec = ImageSpecOwned::new_2d(TypeDesc::FLOAT, size.0, size.1, &["R", "G", "B", "A"]);
        let mut out = ImageOutput::create("kazeharu.exr").unwrap();
        let mut out = out.open(&spec).unwrap();
        out.write_image(data.data()).unwrap();
    }

    #[test]
    fn open_image_psd() {
        let img = ImageInput::open("test_images/cup.psd").unwrap();
        for ch in img.spec().channels() {
            println!("channel {:?}", ch);
        }
    }

    #[test]
    fn open_image_tif() {
        let img = ImageInput::open("test_images/cup.tif").unwrap();
        for ch in img.spec().channels() {
            println!("channel {:?}", ch);
        }
    }

    #[test]
    fn open_nonexistent_image() {
        let img = ImageInput::open("test_images/nonexistent.png");
        if let Err(ref e) = img {
            println!("{}", e);
        }
        assert!(img.is_err());
    }

    #[test]
    fn test_cache_api() {
        let cache = ImageCache::new();

        cache.image("test_images/cup.tif").unwrap();
        cache.image("test_images/cup.tif").unwrap();
        cache.image("test_images/cup.psd").unwrap();
        cache.image("test_images/output0013.exr").unwrap();
        cache.image("test_images/tonberry.jpg").unwrap();
        let img = cache.image("test_images/kazeharu.png").unwrap();

        // members on CachedImage
        img.spec();
        img.width();
        img.height();
        img.depth();
        img.clone().read::<f32>().unwrap();
        img.clone().all_channels();
        img.clone().channels(0..1).unwrap();
        img.clone().channels_by_name(&["R"]).unwrap();
        img.clone().rgba_channels().unwrap();
        img.clone().alpha_channel().unwrap();
        img.clone().all_channels().read::<f32>().unwrap();

        // members on CachedSubimageMipmap
        let sub = img.clone().subimage(0).unwrap();
        sub.spec();
        sub.width();
        sub.height();
        sub.depth();
        sub.read::<f32>().unwrap(); // consumes
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .spec();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .width();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .height();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .depth();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .all_channels();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .channels(0..1)
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .channels_by_name(&["R"])
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .rgba_channels()
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .alpha_channel()
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .read::<f32>()
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage(0)
            .unwrap()
            .all_channels()
            .read::<f32>()
            .unwrap();

        // members on CachedSubimageMipmap (same as above, but through subimage+mipmap)
        let sub = img.clone().subimage_mipmap(0, 0).unwrap();
        sub.spec();
        sub.width();
        sub.height();
        sub.depth();
        sub.read::<f32>().unwrap(); // consumes
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .spec();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .width();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .height();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .depth();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .all_channels();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .channels(0..1)
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .channels_by_name(&["R"])
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .rgba_channels()
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .alpha_channel()
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .read::<f32>()
            .unwrap();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .read::<f32>()
            .unwrap();

        // members of CachedSubimageMipmapChannels
        let chan = img.subimage_mipmap(0, 0).unwrap().all_channels();
        chan.spec();
        chan.width();
        chan.height();
        chan.depth();
        chan.read::<f32>().unwrap(); // consumes.
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .spec();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .width();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .height();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .depth();
        cache
            .image("test_images/kazeharu.png")
            .unwrap()
            .subimage_mipmap(0, 0)
            .unwrap()
            .all_channels()
            .read::<f32>()
            .unwrap();
    }

    #[test]
    fn test_api_overhead() {
        /*let mut img = ImageInput::open("../test_images/kazeharu.png").unwrap();

        for i in 0..100000 {
            img.subimage(0).unwrap().all_channels();
        }*/
    }

}
