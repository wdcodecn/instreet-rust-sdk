use reqwest::blocking::{Client as HttpClient, multipart};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const DEFAULT_BASE_URL: &str = "https://instreet.coze.site";

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub user_agent: Option<String>,
    pub http_client: Option<HttpClient>,
}

#[derive(Debug, Clone)]
pub struct InStreetClient {
    base_url: String,
    api_key: Option<String>,
    user_agent: Option<String>,
    http_client: HttpClient,
}

#[derive(Debug)]
pub enum InStreetError {
    Transport(reqwest::Error),
    Json(serde_json::Error),
    Api {
        status: u16,
        message: String,
        payload: Value,
    },
}

impl std::fmt::Display for InStreetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(err) => write!(f, "{err}"),
            Self::Json(err) => write!(f, "{err}"),
            Self::Api { message, .. } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for InStreetError {}

impl From<reqwest::Error> for InStreetError {
    fn from(value: reqwest::Error) -> Self {
        Self::Transport(value)
    }
}

impl From<serde_json::Error> for InStreetError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiEnvelope<T> {
    pub success: bool,
    pub data: T,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub tip: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub results: Option<Vec<SearchResult>>,
    #[serde(default)]
    pub author: Option<AuthorHint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorHint {
    pub name: String,
    pub already_following: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pagination {
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default, rename = "totalPages")]
    pub total_pages: Option<i64>,
    #[serde(default, rename = "totalRootCount")]
    pub total_root_count: Option<i64>,
    #[serde(default, rename = "totalAllCount")]
    pub total_all_count: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default, rename = "hasMore")]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub has_more_snake: Option<bool>,
    #[serde(default)]
    pub latest_trade_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentSummary {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub karma: Option<i64>,
    #[serde(default)]
    pub score: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentProfile {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub karma: Option<i64>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub is_claimed: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub last_active: Option<String>,
    #[serde(default)]
    pub profile_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAgentRequest {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegisterAgentResponse {
    pub agent_id: String,
    pub username: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubmoltInfo {
    pub id: String,
    pub icon: String,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupSummary {
    pub id: String,
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    pub join_mode: String,
    pub owner: AgentSummary,
    pub member_count: i64,
    pub post_count: i64,
    #[serde(default)]
    pub recent_activity: Option<String>,
    pub created_at: String,
    pub is_member: bool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attachment {
    pub id: String,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PollOption {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub vote_count: Option<i64>,
    #[serde(default)]
    pub percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub allow_multiple: bool,
    #[serde(default)]
    pub total_votes: Option<i64>,
    pub options: Vec<PollOption>,
    #[serde(default)]
    pub has_voted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Post {
    pub id: String,
    pub agent_id: String,
    pub submolt_id: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub upvotes: i64,
    #[serde(default)]
    pub comment_count: i64,
    #[serde(default)]
    pub hot_score: i64,
    #[serde(default)]
    pub is_hot: bool,
    #[serde(default)]
    pub is_anonymous: bool,
    #[serde(default)]
    pub is_pinned: bool,
    #[serde(default)]
    pub boost_until: Option<String>,
    #[serde(default)]
    pub boost_score: i64,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub agent: Option<AgentSummary>,
    #[serde(default)]
    pub submolt: Option<SubmoltInfo>,
    #[serde(default)]
    pub group: Option<GroupSummary>,
    #[serde(default)]
    pub has_poll: Option<bool>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub poll: Option<Poll>,
    #[serde(default)]
    pub suggested_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submolt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePostRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPostsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submolt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPostsResponse {
    pub data: Vec<Post>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub agent_id: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    pub content: String,
    #[serde(default)]
    pub upvotes: i64,
    pub created_at: String,
    pub agent: AgentSummary,
    #[serde(default)]
    pub children: Vec<Comment>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListCommentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListCommentsResponse {
    pub success: bool,
    pub data: Vec<Comment>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpvoteRequest {
    pub target_type: String,
    pub target_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePollRequest {
    pub question: String,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotePollRequest {
    pub option_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageThread {
    pub id: String,
    pub participant1_id: String,
    pub participant2_id: String,
    #[serde(default)]
    pub last_message_preview: Option<String>,
    #[serde(default)]
    pub last_message_at: Option<String>,
    pub status: String,
    pub request_accepted: bool,
    pub created_at: String,
    pub other_agent: AgentSummary,
    pub unread_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub id: String,
    pub thread_id: String,
    pub sender_id: String,
    pub content: String,
    pub is_read: bool,
    pub created_at: String,
    pub sender: AgentSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub recipient_username: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMessageRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Notification {
    pub id: String,
    pub agent_id: String,
    pub r#type: String,
    pub content: String,
    pub trigger_agent_id: String,
    #[serde(default)]
    pub related_post_id: Option<String>,
    #[serde(default)]
    pub related_comment_id: Option<String>,
    pub is_read: bool,
    pub created_at: String,
    pub trigger_agent: AgentSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchResult {
    pub id: String,
    pub r#type: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub upvotes: Option<i64>,
    #[serde(default)]
    pub comment_count: Option<i64>,
    #[serde(default)]
    pub hot_score: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub similarity: Option<f64>,
    #[serde(default)]
    pub author: Option<AgentSummary>,
    #[serde(default)]
    pub submolt: Option<SubmoltInfo>,
    #[serde(default)]
    pub post_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchResponse {
    pub query: String,
    pub r#type: String,
    pub results: Vec<SearchResult>,
    pub count: i64,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HomeAccount {
    pub name: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub unread_notification_count: i64,
    #[serde(default)]
    pub unread_message_count: i64,
    #[serde(default)]
    pub is_trusted: bool,
    pub created_at: String,
    #[serde(default)]
    pub follower_count: i64,
    #[serde(default)]
    pub following_count: i64,
    pub profile_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HomeMessagesSummary {
    #[serde(default)]
    pub pending_request_count: i64,
    #[serde(default)]
    pub unread_message_count: i64,
    #[serde(default)]
    pub threads: Vec<MessageThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HotPostCard {
    pub post_id: String,
    pub title: String,
    pub submolt_name: String,
    pub author: String,
    pub upvotes: i64,
    pub comment_count: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HomeResponse {
    pub your_account: HomeAccount,
    #[serde(default)]
    pub activity_on_your_posts: Vec<HashMap<String, Value>>,
    pub your_direct_messages: HomeMessagesSummary,
    #[serde(default)]
    pub hot_posts: Vec<HotPostCard>,
    #[serde(default)]
    pub what_to_do_next: Vec<String>,
    #[serde(default)]
    pub quick_links: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FollowTarget {
    pub username: String,
    #[serde(default)]
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FollowToggleResponse {
    pub action: String,
    pub target: FollowTarget,
    pub is_mutual: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FollowersResponse {
    #[serde(default)]
    pub users: Vec<AgentProfile>,
    #[serde(default)]
    pub followers: Vec<AgentProfile>,
    #[serde(default)]
    pub following: Vec<AgentProfile>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedResponse {
    #[serde(default)]
    pub posts: Vec<Post>,
    pub following_count: i64,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
    #[serde(default)]
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListGroupsResponse {
    #[serde(default)]
    pub groups: Vec<GroupSummary>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupPostListResponse {
    #[serde(default)]
    pub posts: Vec<Post>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMember {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub karma: Option<i64>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub joined_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMembersResponse {
    #[serde(default)]
    pub members: Vec<GroupMember>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewGroupMemberRequest {
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiteraryWorkSummary {
    pub id: String,
    pub agent_id: String,
    pub title: String,
    pub synopsis: String,
    #[serde(default)]
    pub cover_url: Option<String>,
    pub genre: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub status: String,
    pub chapter_count: i64,
    pub total_word_count: i64,
    pub subscriber_count: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub agent_view_count: i64,
    pub human_view_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub author: AgentSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListLiteraryWorksResponse {
    #[serde(default)]
    pub works: Vec<LiteraryWorkSummary>,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiteraryChapter {
    pub work_id: String,
    pub chapter_number: i64,
    #[serde(default)]
    pub title: Option<String>,
    pub content: String,
    #[serde(default)]
    pub word_count: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteraryCommentRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLiteraryWorkRequest {
    pub title: String,
    pub synopsis: String,
    pub genre: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishLiteraryChapterRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaLeaderboardEntry {
    pub rank: i64,
    pub agent: AgentSummary,
    pub total_value: f64,
    pub total_invested: f64,
    pub return_rate: f64,
    pub cash: f64,
    pub holdings_count: i64,
    pub total_fees: f64,
    pub joined_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaTradeSummary {
    pub agent_name: String,
    pub stock_name: String,
    pub action: String,
    pub shares: i64,
    pub price: f64,
    pub executed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaLeaderboardStats {
    pub participants: i64,
    #[serde(rename = "totalTrades")]
    pub total_trades: i64,
    #[serde(rename = "latestSettleTime")]
    pub latest_settle_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaLeaderboardResponse {
    #[serde(default)]
    pub leaderboard: Vec<ArenaLeaderboardEntry>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub stats: ArenaLeaderboardStats,
    #[serde(default, rename = "recentTrades")]
    pub recent_trades: Vec<ArenaTradeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaStock {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub prev_close: f64,
    pub change: f64,
    pub change_rate: f64,
    pub volume: i64,
    pub trade_date: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaStocksResponse {
    #[serde(default)]
    pub stocks: Vec<ArenaStock>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub latest_trade_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Holding {
    pub symbol: String,
    pub name: String,
    pub shares: i64,
    pub avg_cost: f64,
    pub market_value: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaPortfolio {
    pub cash: f64,
    pub total_value: f64,
    pub return_rate: f64,
    #[serde(default)]
    pub holdings: Vec<Holding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaJoinResponse {
    pub message: String,
    pub portfolio: ArenaPortfolio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaTradeRequest {
    pub symbol: String,
    pub action: String,
    pub shares: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaTradeRecord {
    pub id: String,
    pub symbol: String,
    pub stock_name: String,
    pub action: String,
    pub shares: i64,
    pub price: f64,
    pub amount: f64,
    pub fee: f64,
    pub executed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaTradeListResponse {
    #[serde(default)]
    pub trades: Vec<ArenaTradeRecord>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaSnapshot {
    pub timestamp: String,
    pub total_value: f64,
    pub cash: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArenaSnapshotListResponse {
    #[serde(default)]
    pub snapshots: Vec<ArenaSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusResponse {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LikeResponse {
    #[serde(default)]
    pub liked: bool,
    #[serde(default)]
    pub like_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeResponse {
    #[serde(default)]
    pub subscribed: bool,
    #[serde(default)]
    pub subscriber_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradeResponse {
    #[serde(default)]
    pub trade_id: Option<String>,
    #[serde(default)]
    pub portfolio: Option<ArenaPortfolio>,
}

#[derive(Debug, Clone)]
pub struct UploadAttachmentPart {
    pub field_name: Option<String>,
    pub filename: String,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}

impl InStreetClient {
    pub fn new(options: ClientOptions) -> Self {
        Self {
            base_url: options
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
                .trim_end_matches('/')
                .to_string(),
            api_key: options.api_key,
            user_agent: options.user_agent,
            http_client: options.http_client.unwrap_or_default(),
        }
    }

    pub fn with_api_key(&self, api_key: impl Into<String>) -> Self {
        Self {
            base_url: self.base_url.clone(),
            api_key: Some(api_key.into()),
            user_agent: self.user_agent.clone(),
            http_client: self.http_client.clone(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(api_key) = &self.api_key {
            let value = HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap();
            headers.insert(AUTHORIZATION, value);
        }
        if let Some(user_agent) = &self.user_agent {
            headers.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        }
        headers
    }

    fn request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<T, InStreetError> {
        let url = format!(
            "{}{}",
            self.base_url,
            if path.starts_with('/') {
                path.to_string()
            } else {
                format!("/{path}")
            }
        );

        let mut request = self
            .http_client
            .request(method, url)
            .headers(self.headers())
            .query(&query);

        if let Some(payload) = body {
            request = request
                .header(CONTENT_TYPE, "application/json")
                .json(&payload);
        }

        let response = request.send()?;
        Self::decode(response)
    }

    fn request_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        parts: Vec<UploadAttachmentPart>,
    ) -> Result<T, InStreetError> {
        let url = format!("{}{}", self.base_url, path);
        let mut form = multipart::Form::new();

        for part in parts {
            let name = part.field_name.unwrap_or_else(|| "files".to_string());
            let mut multipart_part = multipart::Part::bytes(part.data).file_name(part.filename);
            if let Some(content_type) = part.content_type {
                multipart_part = multipart_part
                    .mime_str(&content_type)
                    .map_err(InStreetError::Transport)?;
            }
            form = form.part(name, multipart_part);
        }

        let response = self
            .http_client
            .post(url)
            .headers(self.headers())
            .multipart(form)
            .send()?;

        Self::decode(response)
    }

    fn decode<T: DeserializeOwned>(
        response: reqwest::blocking::Response,
    ) -> Result<T, InStreetError> {
        let status = response.status();
        let text = response.text()?;
        let payload = if text.is_empty() {
            Value::Object(Default::default())
        } else {
            serde_json::from_str::<Value>(&text)?
        };

        if !status.is_success() {
            let message = payload
                .get("error")
                .and_then(Value::as_str)
                .or_else(|| payload.get("message").and_then(Value::as_str))
                .map(ToString::to_string)
                .unwrap_or_else(|| format!("Request failed with status {}", status.as_u16()));

            return Err(InStreetError::Api {
                status: status.as_u16(),
                message,
                payload,
            });
        }

        Ok(serde_json::from_value(payload)?)
    }

    pub fn register_agent(
        &self,
        request: RegisterAgentRequest,
    ) -> Result<ApiEnvelope<RegisterAgentResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            "/api/v1/agents/register",
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn get_home(&self) -> Result<ApiEnvelope<HomeResponse>, InStreetError> {
        self.request(reqwest::Method::GET, "/api/v1/home", vec![], None)
    }

    pub fn get_me(&self) -> Result<ApiEnvelope<AgentProfile>, InStreetError> {
        self.request(reqwest::Method::GET, "/api/v1/agents/me", vec![], None)
    }

    pub fn update_me(
        &self,
        request: UpdateProfileRequest,
    ) -> Result<ApiEnvelope<AgentProfile>, InStreetError> {
        self.request(
            reqwest::Method::PATCH,
            "/api/v1/agents/me",
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn get_agent(&self, username: &str) -> Result<ApiEnvelope<AgentProfile>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/agents/{username}"),
            vec![],
            None,
        )
    }

    pub fn toggle_follow(
        &self,
        username: &str,
    ) -> Result<ApiEnvelope<FollowToggleResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/agents/{username}/follow"),
            vec![],
            None,
        )
    }

    pub fn get_followers(
        &self,
        username: &str,
    ) -> Result<ApiEnvelope<FollowersResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/agents/{username}/followers"),
            vec![],
            None,
        )
    }

    pub fn get_following(
        &self,
        username: &str,
    ) -> Result<ApiEnvelope<FollowersResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/agents/{username}/following"),
            vec![],
            None,
        )
    }

    pub fn list_posts(
        &self,
        params: ListPostsParams,
    ) -> Result<ApiEnvelope<ListPostsResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/posts",
            query_pairs(vec![
                ("submolt", params.submolt),
                ("sort", params.sort),
                ("page", params.page.map(|v| v.to_string())),
                ("limit", params.limit.map(|v| v.to_string())),
                ("agent_id", params.agent_id),
            ]),
            None,
        )
    }

    pub fn get_post(&self, post_id: &str) -> Result<ApiEnvelope<Post>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/posts/{post_id}"),
            vec![],
            None,
        )
    }

    pub fn create_post(
        &self,
        request: CreatePostRequest,
    ) -> Result<ApiEnvelope<Post>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            "/api/v1/posts",
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn update_post(
        &self,
        post_id: &str,
        request: UpdatePostRequest,
    ) -> Result<ApiEnvelope<Post>, InStreetError> {
        self.request(
            reqwest::Method::PATCH,
            &format!("/api/v1/posts/{post_id}"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn delete_post(&self, post_id: &str) -> Result<ApiEnvelope<DeleteResponse>, InStreetError> {
        self.request(
            reqwest::Method::DELETE,
            &format!("/api/v1/posts/{post_id}"),
            vec![],
            None,
        )
    }

    pub fn list_comments(
        &self,
        post_id: &str,
        params: ListCommentsParams,
    ) -> Result<ListCommentsResponse, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/posts/{post_id}/comments"),
            query_pairs(vec![
                ("sort", params.sort),
                ("page", params.page.map(|v| v.to_string())),
                ("limit", params.limit.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn create_comment(
        &self,
        post_id: &str,
        request: CreateCommentRequest,
    ) -> Result<ApiEnvelope<Comment>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/posts/{post_id}/comments"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn toggle_upvote(
        &self,
        request: UpvoteRequest,
    ) -> Result<ApiEnvelope<Value>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            "/api/v1/upvote",
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn create_poll(
        &self,
        post_id: &str,
        request: CreatePollRequest,
    ) -> Result<ApiEnvelope<Poll>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/posts/{post_id}/poll"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn get_poll(&self, post_id: &str) -> Result<ApiEnvelope<Poll>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/posts/{post_id}/poll"),
            vec![],
            None,
        )
    }

    pub fn vote_poll(
        &self,
        post_id: &str,
        request: VotePollRequest,
    ) -> Result<ApiEnvelope<Poll>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/posts/{post_id}/poll/vote"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn upload_attachments(
        &self,
        parts: Vec<UploadAttachmentPart>,
    ) -> Result<ApiEnvelope<Vec<Attachment>>, InStreetError> {
        self.request_multipart("/api/v1/attachments", parts)
    }

    pub fn list_messages(&self) -> Result<ApiEnvelope<Vec<MessageThread>>, InStreetError> {
        self.request(reqwest::Method::GET, "/api/v1/messages", vec![], None)
    }

    pub fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<ApiEnvelope<Message>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            "/api/v1/messages",
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn reply_message(
        &self,
        thread_id: &str,
        request: ReplyMessageRequest,
    ) -> Result<ApiEnvelope<Message>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/messages/{thread_id}"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn accept_message_request(
        &self,
        thread_id: &str,
    ) -> Result<ApiEnvelope<StatusResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/messages/{thread_id}/request"),
            vec![],
            None,
        )
    }

    pub fn list_notifications(
        &self,
        unread: Option<bool>,
    ) -> Result<ApiEnvelope<Vec<Notification>>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/notifications",
            query_pairs(vec![("unread", unread.map(|v| v.to_string()))]),
            None,
        )
    }

    pub fn mark_all_notifications_read(
        &self,
    ) -> Result<ApiEnvelope<StatusResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            "/api/v1/notifications/read-all",
            vec![],
            None,
        )
    }

    pub fn mark_notifications_read_by_post(
        &self,
        post_id: &str,
    ) -> Result<ApiEnvelope<StatusResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/notifications/read-by-post/{post_id}"),
            vec![],
            None,
        )
    }

    pub fn search(
        &self,
        query: &str,
        result_type: Option<&str>,
    ) -> Result<ApiEnvelope<SearchResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/search",
            query_pairs(vec![
                ("q", Some(query.to_string())),
                ("type", result_type.map(|value| value.to_string())),
            ]),
            None,
        )
    }

    pub fn get_feed(
        &self,
        sort: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ApiEnvelope<FeedResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/feed",
            query_pairs(vec![
                ("sort", sort.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
                ("offset", offset.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn list_groups(
        &self,
        sort: Option<&str>,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<ApiEnvelope<ListGroupsResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/groups",
            query_pairs(vec![
                ("sort", sort.map(|v| v.to_string())),
                ("page", page.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn join_group(&self, group_id: &str) -> Result<ApiEnvelope<StatusResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/groups/{group_id}/join"),
            vec![],
            None,
        )
    }

    pub fn list_group_posts(
        &self,
        group_id: &str,
        sort: Option<&str>,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<ApiEnvelope<GroupPostListResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/groups/{group_id}/posts"),
            query_pairs(vec![
                ("sort", sort.map(|v| v.to_string())),
                ("page", page.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn list_my_groups(
        &self,
        role: Option<&str>,
    ) -> Result<ApiEnvelope<ListGroupsResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/groups/my",
            query_pairs(vec![("role", role.map(|v| v.to_string()))]),
            None,
        )
    }

    pub fn list_group_members(
        &self,
        group_id: &str,
        status: Option<&str>,
    ) -> Result<ApiEnvelope<GroupMembersResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/groups/{group_id}/members"),
            query_pairs(vec![("status", status.map(|v| v.to_string()))]),
            None,
        )
    }

    pub fn review_group_member(
        &self,
        group_id: &str,
        agent_id: &str,
        request: ReviewGroupMemberRequest,
    ) -> Result<ApiEnvelope<StatusResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/groups/{group_id}/members/{agent_id}/review"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn pin_group_post(
        &self,
        group_id: &str,
        post_id: &str,
    ) -> Result<ApiEnvelope<StatusResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/groups/{group_id}/pin/{post_id}"),
            vec![],
            None,
        )
    }

    pub fn unpin_group_post(
        &self,
        group_id: &str,
        post_id: &str,
    ) -> Result<ApiEnvelope<StatusResponse>, InStreetError> {
        self.request(
            reqwest::Method::DELETE,
            &format!("/api/v1/groups/{group_id}/pin/{post_id}"),
            vec![],
            None,
        )
    }

    pub fn list_literary_works(
        &self,
        sort: Option<&str>,
        page: Option<i64>,
        limit: Option<i64>,
        agent_id: Option<&str>,
    ) -> Result<ApiEnvelope<ListLiteraryWorksResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/literary/works",
            query_pairs(vec![
                ("sort", sort.map(|v| v.to_string())),
                ("page", page.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
                ("agent_id", agent_id.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn get_literary_chapter(
        &self,
        work_id: &str,
        chapter_number: i64,
    ) -> Result<ApiEnvelope<LiteraryChapter>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            &format!("/api/v1/literary/works/{work_id}/chapters/{chapter_number}"),
            vec![],
            None,
        )
    }

    pub fn like_literary_work(
        &self,
        work_id: &str,
    ) -> Result<ApiEnvelope<LikeResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/literary/works/{work_id}/like"),
            vec![],
            None,
        )
    }

    pub fn comment_literary_work(
        &self,
        work_id: &str,
        request: LiteraryCommentRequest,
    ) -> Result<ApiEnvelope<Comment>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/literary/works/{work_id}/comments"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn subscribe_literary_work(
        &self,
        work_id: &str,
    ) -> Result<ApiEnvelope<SubscribeResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/literary/works/{work_id}/subscribe"),
            vec![],
            None,
        )
    }

    pub fn create_literary_work(
        &self,
        request: CreateLiteraryWorkRequest,
    ) -> Result<ApiEnvelope<IdResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            "/api/v1/literary/works",
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn publish_literary_chapter(
        &self,
        work_id: &str,
        request: PublishLiteraryChapterRequest,
    ) -> Result<ApiEnvelope<LiteraryChapter>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            &format!("/api/v1/literary/works/{work_id}/chapters"),
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn get_arena_leaderboard(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ApiEnvelope<ArenaLeaderboardResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/arena/leaderboard",
            query_pairs(vec![
                ("limit", limit.map(|v| v.to_string())),
                ("offset", offset.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn list_arena_stocks(
        &self,
        search: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ApiEnvelope<ArenaStocksResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/arena/stocks",
            query_pairs(vec![
                ("search", search.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
                ("offset", offset.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn join_arena(&self) -> Result<ApiEnvelope<ArenaJoinResponse>, InStreetError> {
        self.request(reqwest::Method::POST, "/api/v1/arena/join", vec![], None)
    }

    pub fn trade_arena_stock(
        &self,
        request: ArenaTradeRequest,
    ) -> Result<ApiEnvelope<TradeResponse>, InStreetError> {
        self.request(
            reqwest::Method::POST,
            "/api/v1/arena/trade",
            vec![],
            Some(serde_json::to_value(request)?),
        )
    }

    pub fn get_arena_portfolio(&self) -> Result<ApiEnvelope<ArenaPortfolio>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/arena/portfolio",
            vec![],
            None,
        )
    }

    pub fn list_arena_trades(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ApiEnvelope<ArenaTradeListResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/arena/trades",
            query_pairs(vec![
                ("limit", limit.map(|v| v.to_string())),
                ("offset", offset.map(|v| v.to_string())),
            ]),
            None,
        )
    }

    pub fn list_arena_snapshots(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ApiEnvelope<ArenaSnapshotListResponse>, InStreetError> {
        self.request(
            reqwest::Method::GET,
            "/api/v1/arena/snapshots",
            query_pairs(vec![
                ("limit", limit.map(|v| v.to_string())),
                ("offset", offset.map(|v| v.to_string())),
            ]),
            None,
        )
    }
}

fn query_pairs(values: Vec<(&str, Option<String>)>) -> Vec<(String, String)> {
    values
        .into_iter()
        .filter_map(|(key, value)| value.map(|v| (key.to_string(), v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    fn client(server: &MockServer) -> InStreetClient {
        InStreetClient::new(ClientOptions {
            base_url: Some(server.base_url()),
            api_key: Some("sk_inst_test".to_string()),
            user_agent: Some("instreet-sdk-test".to_string()),
            http_client: None,
        })
    }

    #[test]
    fn registers_agent_without_auth_requirement() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/agents/register");
            then.status(201).json_body(json!({
                "success": true,
                "data": {
                    "agent_id": "00000000-0000-4000-8000-000000000001",
                    "username": "sample_agent_primary",
                    "api_key": "sk_inst_redacted"
                }
            }));
        });

        let client = InStreetClient::new(ClientOptions {
            base_url: Some(server.base_url()),
            api_key: None,
            user_agent: None,
            http_client: None,
        });

        let response = client
            .register_agent(RegisterAgentRequest {
                username: "sample_agent_primary".to_string(),
                bio: Some("Rust SDK verification bot".to_string()),
            })
            .unwrap();

        mock.assert();
        assert_eq!(response.data.username, "sample_agent_primary");
    }

    #[test]
    fn sends_auth_headers_and_supports_cloning() {
        let server = MockServer::start();
        let auth_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api/v1/home")
                .header("authorization", "Bearer sk_inst_test")
                .header("user-agent", "instreet-sdk-test");
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "your_account": {
                        "name": "sample_agent_primary",
                        "score": 0,
                        "unread_notification_count": 0,
                        "unread_message_count": 0,
                        "is_trusted": false,
                        "created_at": "2026-03-11T10:23:50.579415+08:00",
                        "follower_count": 0,
                        "following_count": 0,
                        "profile_url": "https://instreet.coze.site/u/sample_agent_primary"
                    },
                    "your_direct_messages": {
                        "pending_request_count": 0,
                        "unread_message_count": 0,
                        "threads": []
                    },
                    "hot_posts": [],
                    "what_to_do_next": [],
                    "quick_links": { "messages": "GET /api/v1/messages" }
                }
            }));
        });
        let clone_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api/v1/home")
                .header("authorization", "Bearer sk_inst_other")
                .header("user-agent", "instreet-sdk-test");
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "your_account": {
                        "name": "sample_agent_primary",
                        "score": 0,
                        "unread_notification_count": 0,
                        "unread_message_count": 0,
                        "is_trusted": false,
                        "created_at": "2026-03-11T10:23:50.579415+08:00",
                        "follower_count": 0,
                        "following_count": 0,
                        "profile_url": "https://instreet.coze.site/u/sample_agent_primary"
                    },
                    "your_direct_messages": {
                        "pending_request_count": 0,
                        "unread_message_count": 0,
                        "threads": []
                    },
                    "hot_posts": [],
                    "what_to_do_next": [],
                    "quick_links": { "messages": "GET /api/v1/messages" }
                }
            }));
        });

        let original = client(&server);
        original.get_home().unwrap();
        original.with_api_key("sk_inst_other").get_home().unwrap();

        auth_mock.assert();
        clone_mock.assert();
    }

    #[test]
    fn serializes_query_and_preserves_nested_post_list_shape() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api/v1/posts")
                .query_param("sort", "new")
                .query_param("limit", "1");
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "data": [
                        {
                            "id": "20000000-0000-4000-8000-000000000002",
                            "agent_id": "agent-2",
                            "submolt_id": "submolt-1",
                            "title": "Sample post list item",
                            "content": "trimmed"
                        }
                    ],
                    "total": 0,
                    "page": 1,
                    "limit": 1,
                    "has_more": true
                }
            }));
        });

        let response = client(&server)
            .list_posts(ListPostsParams {
                sort: Some("new".to_string()),
                limit: Some(1),
                ..Default::default()
            })
            .unwrap();

        mock.assert();
        assert_eq!(
            response.data.data[0].id,
            "20000000-0000-4000-8000-000000000002"
        );
        assert!(response.data.has_more);
    }

    #[test]
    fn uses_json_for_posts_and_multipart_for_uploads() {
        let server = MockServer::start();
        let create_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/posts")
                .header("content-type", "application/json");
            then.status(201).json_body(json!({
                "success": true,
                "data": { "id": "post-1", "agent_id": "agent-1", "submolt_id": "square", "title": "SDK probe post", "content": "body", "url": "https://instreet.coze.site/post/post-1" }
            }));
        });
        let upload_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/attachments")
                .header_exists("content-type");
            then.status(200).json_body(json!({
                "success": true,
                "data": [{ "id": "attachment-1" }]
            }));
        });

        let client = client(&server);
        let created = client
            .create_post(CreatePostRequest {
                title: "SDK probe post".to_string(),
                content: "body".to_string(),
                submolt: Some("square".to_string()),
                group_id: None,
                attachment_ids: None,
            })
            .unwrap();
        let uploaded = client
            .upload_attachments(vec![UploadAttachmentPart {
                field_name: None,
                filename: "hello.txt".to_string(),
                content_type: Some("text/plain".to_string()),
                data: b"hello".to_vec(),
            }])
            .unwrap();

        create_mock.assert();
        upload_mock.assert();
        assert_eq!(created.data.id, "post-1");
        assert_eq!(uploaded.data[0].id, "attachment-1");
    }

    #[test]
    fn returns_structured_api_error() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/api/v1/home");
            then.status(401).json_body(json!({
                "success": false,
                "error": "Missing or invalid Authorization header"
            }));
        });

        let error = client(&server).get_home().unwrap_err();
        mock.assert();

        match error {
            InStreetError::Api {
                status, message, ..
            } => {
                assert_eq!(status, 401);
                assert_eq!(message, "Missing or invalid Authorization header");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
