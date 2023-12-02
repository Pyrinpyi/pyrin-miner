use crate::proto::{
    pyipad_message::Payload, GetBlockTemplateRequestMessage, GetInfoRequestMessage, PyipadMessage,
    NotifyBlockAddedRequestMessage, NotifyNewBlockTemplateRequestMessage, RpcBlock, SubmitBlockRequestMessage,
};
use crate::{
    pow::{self, HeaderHasher},
    Hash,
};

impl PyipadMessage {
    #[inline(always)]
    pub fn get_info_request() -> Self {
        PyipadMessage { payload: Some(Payload::GetInfoRequest(GetInfoRequestMessage {})) }
    }
    #[inline(always)]
    pub fn notify_block_added() -> Self {
        PyipadMessage { payload: Some(Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage {})) }
    }

    #[inline(always)]
    pub fn submit_block(block: RpcBlock) -> Self {
        PyipadMessage {
            payload: Some(Payload::SubmitBlockRequest(SubmitBlockRequestMessage {
                block: Some(block),
                allow_non_daa_blocks: false,
            })),
        }
    }
}

impl From<GetInfoRequestMessage> for PyipadMessage {
    fn from(a: GetInfoRequestMessage) -> Self {
        PyipadMessage { payload: Some(Payload::GetInfoRequest(a)) }
    }
}
impl From<NotifyBlockAddedRequestMessage> for PyipadMessage {
    fn from(a: NotifyBlockAddedRequestMessage) -> Self {
        PyipadMessage { payload: Some(Payload::NotifyBlockAddedRequest(a)) }
    }
}

impl From<GetBlockTemplateRequestMessage> for PyipadMessage {
    fn from(a: GetBlockTemplateRequestMessage) -> Self {
        PyipadMessage { payload: Some(Payload::GetBlockTemplateRequest(a)) }
    }
}

impl From<NotifyNewBlockTemplateRequestMessage> for PyipadMessage {
    fn from(a: NotifyNewBlockTemplateRequestMessage) -> Self {
        PyipadMessage { payload: Some(Payload::NotifyNewBlockTemplateRequest(a)) }
    }
}

impl RpcBlock {
    #[inline(always)]
    pub fn block_hash(&self) -> Option<Hash> {
        let mut hasher = HeaderHasher::new();
        pow::serialize_header(&mut hasher, self.header.as_ref()?, false);
        Some(hasher.finalize())
    }
}
