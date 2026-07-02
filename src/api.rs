use crate::YeepayClient;
use crate::data::order_divide::{OrderDivideCompleteReq, OrderDivideCompleteResp};
use crate::data::order_query::{OrderQueryReq, OrderQueryResp};
use yeepay_core::{YeepayClientTrait, get, post_form};

get!(
    YeepayClient,order_query,"/rest/v1.0/trade/order/query",OrderQueryReq,OrderQueryResp
);

post_form!(
    YeepayClient,order_divide_complete,"/rest/v1.0/divide/complete",OrderDivideCompleteReq,OrderDivideCompleteResp
);
