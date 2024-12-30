use crate::error::NightfallError;

use super::ProfileContext;
use super::ProfileType;
use super::StreamType;
use super::TranscodingProfile;

#[derive(Debug)]
pub struct ThumbnailProfile;

impl TranscodingProfile for ThumbnailProfile {
    fn profile_type(&self) -> ProfileType {
        ProfileType::Transcode
    }

    fn stream_type(&self) -> StreamType {
        StreamType::Thumbnail
    }

    fn name(&self) -> &str {
        "ThumbnailProfile"
    }

    fn build(&self, ctx: ProfileContext) -> Option<Vec<String>> {
        let args = vec![
            "-y".into(),
            "-skip_frame".into(),
            "nokey".into(),
            "-i".into(),
            ctx.file,
            "-vf".into(),
            // Make a thumbnail every 2 seconds that is 320px in width
            "fps=1/2,scale=min(320\\,iw):ow/dar,tile=8x6".into(),
            "-c:v".into(),
            "mjpeg".into(),
            "-fps_mode".into(),
            "passthrough".into(),
            "-qscale:v".into(),
            "2".into(),
            format!("{}/%010d.jpg", ctx.output_ctx.outdir),
        ];

        Some(args)
    }

    fn supports(&self, ctx: &ProfileContext) -> Result<(), NightfallError> {
        if ctx.output_ctx.height.is_some()
            || ctx.output_ctx.width.is_some()
            || ctx.output_ctx.bitrate.is_some()
        {
            return Err(NightfallError::ProfileNotSupported(
                "Thumbnails must be generated from a video input source.".into(),
            ));
        }

        if ctx.output_ctx.codec == "jpg" {
            return Ok(());
        }

        Err(NightfallError::ProfileNotSupported(format!(
            "Codec {} not supported.",
            ctx.input_ctx.codec.as_str()
        )))
    }

    fn tag(&self) -> &str {
        "jpg"
    }
}
