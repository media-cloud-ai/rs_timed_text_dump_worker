#[macro_use]
extern crate serde_derive;

use std::{
  ops::Deref,
  sync::{mpsc::Sender, Arc, Mutex},
};

use mcai_worker_sdk::{
  info, job::JobResult, FormatContext, MessageError, MessageEvent, ProcessFrame, ProcessResult,
  Result, StreamDescriptor, Version,
};
use schemars::JsonSchema;
use stainless_ffmpeg_sys::AVMediaType;

pub mod built_info {
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TTDumpWorkerParameters {
  source_path: Option<String>,
  destination_path: Option<String>,
}

#[derive(Debug, Default)]
struct TTDumpWorkerContext {}

impl MessageEvent<TTDumpWorkerParameters> for TTDumpWorkerContext {
  fn get_name(&self) -> String {
    "TT Dump worker".to_string()
  }

  fn get_short_description(&self) -> String {
    "Timed text dump example worker".to_string()
  }

  fn get_description(&self) -> String {
    "This worker read timed text from a specified SRT stream and dump the read content.".to_string()
  }

  fn get_version(&self) -> Version {
    Version::parse(built_info::PKG_VERSION).expect("unable to locate Package version")
  }

  fn init(&mut self) -> Result<()> {
    Ok(())
  }

  fn init_process(
    &mut self,
    _parameters: TTDumpWorkerParameters,
    format_context: Arc<Mutex<FormatContext>>,
    _result: Arc<Mutex<Sender<ProcessResult>>>,
  ) -> Result<Vec<StreamDescriptor>> {
    let mut stream_descriptors = vec![];

    let format_context = format_context.lock().unwrap();
    for stream_index in 0..format_context.get_nb_streams() {
      let stream_type = format_context.get_stream_type(stream_index as isize);
      info!(
        "Handle stream #{} with type: {:?}",
        stream_index, stream_type
      );

      match stream_type {
        AVMediaType::AVMEDIA_TYPE_SUBTITLE => {
          stream_descriptors.push(StreamDescriptor::new_data(stream_index as usize))
        }
        AVMediaType::AVMEDIA_TYPE_DATA => {
          stream_descriptors.push(StreamDescriptor::new_data(stream_index as usize))
        }
        _ => info!("Skip stream #{}", stream_index),
      };
    }
    Ok(stream_descriptors)
  }

  fn process_frame(
    &mut self,
    _job_result: JobResult,
    _stream_index: usize,
    frame: ProcessFrame,
  ) -> Result<ProcessResult> {
    match frame {
      ProcessFrame::EbuTtmlLive(ebu_ttml_live) => {
        let ebu_ttml = ebu_ttml_live.deref().clone();

        let xml = yaserde::ser::to_string(&ebu_ttml).unwrap();
        println!("{}", xml);

        Ok(ProcessResult::new_xml(ebu_ttml))
      }
      _ => Err(MessageError::NotImplemented()),
    }
  }

  fn ending_process(&mut self) -> Result<()> {
    Ok(())
  }
}

fn main() {
  let worker_context = TTDumpWorkerContext::default();
  mcai_worker_sdk::start_worker(worker_context);
}
