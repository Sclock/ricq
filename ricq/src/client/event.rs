use std::sync::Arc;

use ricq_core::command::profile_service::{JoinGroupRequest, NewFriendRequest, SelfInvited};
use ricq_core::structs::{
    DeleteFriend, FriendAudioMessage, FriendInfo, FriendMessageRecall, FriendPoke,
    GroupAudioMessage, GroupDisband, GroupLeave, GroupMessageRecall, GroupMute, GroupNameUpdate,
    GroupTempMessage, MemberPermissionChange, NewMember,
};
use ricq_core::{jce, RQResult};

use crate::structs::{FriendMessage, GroupMessage};
use crate::Client;

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: GroupMessage,
}

impl GroupMessageEvent {
    pub async fn recall(&self) -> RQResult<()> {
        // TODO check permission
        self.client
            .recall_group_message(
                self.message.group_code,
                self.message.seqs.clone(),
                self.message.rands.clone(),
            )
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: FriendMessage,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupTempMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: GroupTempMessage,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupRequestEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: JoinGroupRequest,
}

impl GroupRequestEvent {
    pub async fn accept(&self) -> RQResult<()> {
        self.client
            .solve_group_system_message(
                self.request.msg_seq,
                self.request.req_uin,
                self.request.group_code,
                self.request.suspicious,
                self.request.invitor_uin.is_some(),
                true,
                false,
                "".into(),
            )
            .await
    }

    pub async fn reject(&self, reason: String, block: bool) -> RQResult<()> {
        self.client
            .solve_group_system_message(
                self.request.msg_seq,
                self.request.req_uin,
                self.request.group_code,
                self.request.suspicious,
                self.request.invitor_uin.is_some(),
                false,
                block,
                reason,
            )
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendRequestEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: NewFriendRequest,
}

impl FriendRequestEvent {
    pub async fn accept(&self) -> RQResult<()> {
        self.client
            .solve_friend_system_message(self.request.msg_seq, self.request.req_uin, true)
            .await
    }

    pub async fn reject(&self) -> RQResult<()> {
        self.client
            .solve_friend_system_message(self.request.msg_seq, self.request.req_uin, false)
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct NewMemberEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub new_member: NewMember,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMuteEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub group_mute: GroupMute,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendMessageRecallEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub recall: FriendMessageRecall,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMessageRecallEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub recall: GroupMessageRecall,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct NewFriendEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub friend: FriendInfo,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupLeaveEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub leave: GroupLeave,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupDisbandEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub disband: GroupDisband,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendPokeEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub poke: FriendPoke,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupNameUpdateEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub update: GroupNameUpdate,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct DeleteFriendEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub delete: DeleteFriend,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct MemberPermissionChangeEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub change: MemberPermissionChange,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct SelfInvitedEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: SelfInvited,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupAudioMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: GroupAudioMessage,
}

impl GroupAudioMessageEvent {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_group_audio_url(self.message.group_code, self.message.audio.clone())
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendAudioMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: FriendAudioMessage,
}

impl FriendAudioMessageEvent {
    pub async fn url(&self) -> RQResult<String> {
        self.client
            .get_friend_audio_url(self.message.from_uin, self.message.audio.clone())
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct KickedOfflineEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub offline: jce::RequestPushForceOffline,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct MSFOfflineEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub offline: jce::RequestMSFForceOffline,
}
