use crate::{
    accounting::orders::AssetType,
    responses::{
        account::{GetPositionsResp, StreamPositionsResp},
        ApiResponse,
    },
    Client, Error,
};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The open trades (positons).
pub struct Position {
    #[serde(rename = "AccountID")]
    /// The `Account` id the `Position` belongs to.
    pub account_id: String,

    /// Indicates the asset type of the position.
    pub asset_type: AssetType,

    /// The average price of the position currently held.
    pub average_price: String,

    /// The highest price a prospective buyer is prepared to pay at
    /// a particular time for a trading unit of a given symbol.
    pub bid: String,

    /// The price at which a security, futures contract, or other
    /// financial instrument is offered for sale.
    pub ask: String,

    /// The currency conversion rate that is used in order to convert
    /// from the currency of the symbol to the currency of the account.
    pub conversion_rate: String,

    /// DayTradeMargin used on open positions.
    ///
    /// NOTE: Currently only calculated for futures positions.
    /// Other asset classes will have a 0 for this value.
    pub day_trade_requirement: String,

    /// The UTC formatted expiration date of the future or option symbol,
    /// in the country the contract is traded in.
    ///
    /// NOTE: The time portion of the value should be ignored.
    pub expiration_date: Option<String>,

    /// The margin account balance denominated in the symbol currency required
    /// for entering a position on margin.
    ///
    /// NOTE: Only applies to future and option positions.
    pub initial_requirement: String,

    /// The last price at which the symbol traded.
    pub last: String,

    /// Specifies if the position is Long or Short.
    pub long_short: PositionType,

    /// The MarkToMarketPrice value is the weighted average of the previous close
    /// price for the position quantity held overnight and the purchase price of the
    /// position quantity opened during the current market session.
    ///
    /// NOTE: This value is used to calculate TodaysProfitLoss.
    ///
    /// NOTE: Only applies to equity and option positions.
    pub mark_to_market_price: String,

    /// The actual market value denominated in the symbol currency of the open position.
    ///
    /// NOTE: This value is updated in real-time.
    pub market_value: String,

    #[serde(rename = "PositionID")]
    /// A unique identifier for the position.
    pub position_id: String,

    /// The number of shares or contracts for a particular position.
    ///
    /// NOTE: This value is negative for short positions.
    pub quantity: String,

    /// Symbol of the position.
    pub symbol: String,

    /// Time the position was entered.
    pub timestamp: String,

    /// The unrealized profit or loss denominated in the account currency on the position
    /// held, calculated using the MarkToMarketPrice.
    ///
    /// NOTE: Only applies to equity and option positions.
    #[serde(rename = "TodaysProfitLoss")]
    pub todays_pnl: String,

    /// The total cost denominated in the account currency of the open position.
    pub total_cost: String,

    #[serde(rename = "UnrealizedProfitLoss")]
    /// The unrealized profit or loss denominated in the symbol currency on the position
    /// held, calculated based on the average price of the position.
    pub unrealized_pnl: String,

    #[serde(rename = "UnrealizedProfitLossPercent")]
    /// The unrealized profit or loss on the position expressed as a percentage of the
    /// initial value of the position.
    pub unrealized_pnl_percent: String,

    #[serde(rename = "UnrealizedProfitLossQty")]
    /// The unrealized profit or loss denominated in the account currency divided by the
    /// number of shares, contracts or units held.
    pub unrealized_pnl_qty: String,
}
impl Position {
    /// Fetches positions for the given `Account`.
    pub(super) async fn get_by_account<S: Into<String>>(
        account_id: S,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!("brokerage/accounts/{}/positions", account_id.into());

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetPositionsResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.positions),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches specific `Position`(s) by their id for the `Account`.
    pub(super) async fn find<S: Into<String>>(
        position_ids: Vec<S>,
        account_id: String,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!("brokerage/accounts/{account_id}/positions");

        let position_ids: Vec<String> = position_ids.into_iter().map(|id| id.into()).collect();

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetPositionsResp>>()
            .await?
        {
            ApiResponse::Success(resp) => {
                let positions: Vec<Position> = resp
                    .positions
                    .into_iter()
                    .filter(|position| position_ids.contains(&position.position_id))
                    .collect();

                Ok(positions)
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches specific `Position`(s) by their id for the `Account`(s).
    pub(super) async fn find_in_accounts<S: Into<String>>(
        position_ids: Vec<S>,
        account_ids: Vec<S>,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/positions",
            account_ids
                .into_iter()
                .map(|id| id.into())
                .collect::<Vec<String>>()
                .join(",")
        );

        let position_ids: Vec<String> = position_ids.into_iter().map(|id| id.into()).collect();

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetPositionsResp>>()
            .await?
        {
            ApiResponse::Success(resp) => {
                let positions: Vec<Position> = resp
                    .positions
                    .into_iter()
                    .filter(|position| position_ids.contains(&position.position_id))
                    .collect();

                Ok(positions)
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches positions for the given `Account`.
    pub(super) async fn get_by_symbols<S: Into<String>>(
        symbols: S,
        account_id: S,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/positions?symbol={}",
            account_id.into(),
            symbols.into()
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetPositionsResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.positions),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches positions for the given `Account`(s).
    pub(super) async fn get_by_symbols_and_accounts(
        symbols: &str,
        account_ids: Vec<&str>,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/positions?symbol={}",
            account_ids.join(","),
            symbols
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetPositionsResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.positions),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches positions for the given `Account`(s).
    pub(super) async fn get_by_accounts<S: Into<String>>(
        account_ids: Vec<S>,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/positions",
            account_ids
                .into_iter()
                .map(|account_id| account_id.into())
                .collect::<Vec<String>>()
                .join(",")
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetPositionsResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.positions),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Stream `Position`s for the given `Account`.
    pub(super) fn stream<S: Into<String>>(
        account_id: S,
        client: &Client,
    ) -> impl Stream<Item = Result<StreamPositionsResp, Error>> + '_ {
        let endpoint = format!("brokerage/stream/accounts/{}/positions", account_id.into());

        client.stream(endpoint).filter_map(|chunk| async {
            match chunk {
                Ok(value) => match serde_json::from_value::<StreamPositionsResp>(value) {
                    Ok(stream_positions_chunk) => Some(Ok(stream_positions_chunk)),
                    Err(e) => Some(Err(Error::Json(e))),
                },
                Err(e) => Some(Err(e)),
            }
        })
    }

    /// Stream `Position`s for the given `Account`(s).
    pub(super) fn stream_for_accounts<S: Into<String>>(
        account_ids: Vec<S>,
        client: &Client,
    ) -> impl Stream<Item = Result<StreamPositionsResp, Error>> + '_ {
        let endpoint = format!(
            "brokerage/stream/accounts/{}/positions",
            account_ids
                .into_iter()
                .map(|id| id.into())
                .collect::<Vec<String>>()
                .join(",")
        );

        client.stream(endpoint).filter_map(|chunk| async {
            match chunk {
                Ok(value) => match serde_json::from_value::<StreamPositionsResp>(value) {
                    Ok(stream_positions_chunk) => Some(Ok(stream_positions_chunk)),
                    Err(e) => Some(Err(Error::Json(e))),
                },
                Err(e) => Some(Err(e)),
            }
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
/// A position type can either be short or long
pub enum PositionType {
    /// Long a share, or futures/options contract
    Long,

    /// Short a share, or futures/options contract
    Short,
}

impl Client {
    /// Fetches a specific `Position` by it's id for a given `Account` id.
    ///
    /// # Example
    /// ---
    ///
    /// Grab a specific position, say you need to check for updates on some
    /// position and you already know it's position id and account id, here's
    /// how you would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// let position = client.get_position("YOUR_POSITION_ID", "YOUR_ACCOUNT_ID").await?;
    /// println!("Position: {position:?}");
    /// ```
    pub async fn get_position<S: Into<String>>(
        &self,
        position_id: S,
        account_id: S,
    ) -> Result<Position, Error> {
        let positions = Position::find(vec![position_id], account_id.into(), self).await?;

        Ok(positions[0].clone())
    }

    /// Fetches a specific `Position` by it's id for given `Account` id's.
    ///
    /// # Example
    /// ---
    ///
    /// Grab a specific position, say you need to check for updates on some
    /// position and you already know it's position id but not the account id,
    /// here's how you would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// let position = client
    ///     .get_position_in_accounts(
    ///         "YOUR_POSITION_ID",
    ///         vec!["YOUR_ACCOUNT_ID_1", "YOUR_ACCOUNT_ID_N"]
    ///     ).await?;
    /// println!("Position: {position:?}");
    /// ```
    pub async fn get_position_in_accounts<S: Into<String>>(
        &self,
        position_id: S,
        account_ids: Vec<S>,
    ) -> Result<Position, Error> {
        let positions = Position::find_in_accounts(vec![position_id], account_ids, self).await?;

        Ok(positions[0].clone())
    }

    /// Fetches all `Position`(s) for a given `Account` id.
    ///
    /// # Example
    /// ---
    ///
    /// Grab a specific position, say you need to check for updates on some
    /// position and you already know it's position id and account id, here's
    /// how you would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// let positions = client.get_positions_in_account("YOUR_ACCOUNT_ID").await?;
    /// println!("Position: {position:?}");
    /// ```
    pub async fn get_positions_in_account<S: Into<String>>(
        &self,
        account_id: S,
    ) -> Result<Vec<Position>, Error> {
        Position::get_by_account(account_id, self).await
    }

    /// Fetches all `Position`(s) for the given `Account` id's.
    ///
    /// # Example
    /// ---
    ///
    /// Grab all the positions in 2 different accounts.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// let positions = client
    ///     .get_positions_in_accounts(vec!["YOUR_ACCOUNT_ID_1", "YOUR_ACCOUNT_ID_2"])
    ///     .await?;
    /// println!("Positions: {positions:?}");
    /// ```
    pub async fn get_positions_in_accounts<S: Into<String>>(
        &self,
        account_ids: Vec<S>,
    ) -> Result<Vec<Position>, Error> {
        Position::get_by_accounts(account_ids, self).await
    }

    /// Fetches specific `Position`(s) by their id for the given `Account` id.
    ///
    /// # Example
    /// ---
    ///
    /// Grab specific positions, say you need to check for updates on 2 specific
    /// positions and you already know their position ids, here's how you would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// let positions = client
    ///     .get_positions_by_id(
    ///         vec!["YOUR_POSITION_ID_1", "YOUR_POSITION_ID_2"],
    ///         "YOUR_ACCOUNT_ID",
    ///     )
    ///     .await?;
    /// println!("Positions: {positions:?}");
    /// ```
    pub async fn get_positions_by_id<S: Into<String>>(
        &self,
        position_ids: Vec<S>,
        account_id: S,
    ) -> Result<Vec<Position>, Error> {
        Position::find(position_ids, account_id.into(), self).await
    }

    /// Fetches specific `Position`(s) by their id for the given `Account` id's.
    ///
    /// # Example
    /// ---
    ///
    /// Grab specific positions for specific accounts.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// let positions = client
    ///     .get_positions_by_id(
    ///         vec!["YOUR_POSITION_ID_1", "YOUR_POSITION_ID_2"],
    ///         "YOUR_ACCOUNT_ID",
    ///     )
    ///     .await?;
    /// println!("Positions: {positions:?}");
    /// ```
    pub async fn get_positions_by_id_in_accounts<S: Into<String>>(
        &self,
        account_ids: Vec<S>,
        position_ids: Vec<S>,
    ) -> Result<Vec<Position>, Error> {
        Position::find_in_accounts(position_ids, account_ids, self).await
    }
}
