use crate::YeepayClient;
use crate::data::GlobalMerchantWrap;
use crate::data::order_create::*;
use crate::data::order_divide::*;
use crate::data::order_query::*;
use crate::data::order_refund::*;
use crate::data::wallet::*;
use yeepay_core::{YeepayClientTrait, get, post_form};

get!(
    /// # 钱包账户信息查询
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/get__rest__v1.0__m-wallet__member__query
    ///
    /// - body: [WalletQueryReq]
    /// - return: [WalletQueryResp]
    YeepayClient,
    wallet_member_query,
    "/rest/v1.0/m-wallet/member/query",
    GlobalMerchantWrap<WalletQueryReq>,
    WalletQueryResp
);

post_form!(
    /// # 钱包注册/登录接口
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v2.0__m-wallet__wallet__index
    ///
    /// - body: [WalletIndexReq]
    /// - return: [WalletIndexResp]
    YeepayClient,
    wallet,
    "/rest/v2.0/m-wallet/wallet/index",
    GlobalMerchantWrap<WalletIndexReq>,
    WalletIndexResp
);

post_form!(
    /// # 注销会员钱包
    ///
    /// 商户注销会员钱包功能
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__wallet__cancel
    ///
    /// - body: [WalletIndexReq]
    /// - return: [WalletIndexResp]
    ///
    /// 钱包注销需知：
    ///
    /// - 1.注销前，用户需完成解绑全部银行卡
    /// - 2.注销前，用户需保证钱包余额已清零并无在途提现资金
    /// - 3.钱包注销后，将无法使用本钱包，包括绑卡、充值、提现、支付、收款等功能
    /// - 4.钱包注销后，将无法再次找回曽添加或绑定的任何内容或信息，包括不限于交易记录
    /// - 5.钱包注销后，该平台的此账号将无法再次开通钱包，请谨慎操作
    /// - 6.如需接收注销通知，需提供通知地址
    ///
    YeepayClient,
    wallet_cancel,
    "/rest/v1.0/m-wallet/wallet/cancel",
    GlobalMerchantWrap<WalletCancelReq>,
    WalletCancelResp
);

post_form!(
    /// # 营销红包转账
    ///
    /// 商户与用户之间发生的交易行为，商户可向用户的钱包账户转账，商户调用此接口完成转账行为
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__transfer__b2c__market
    ///
    /// - body: [TransferB2CMarketReq]
    /// - return: [TransferB2CMarketResp]
    ///
    YeepayClient,
    transfer_b2c_market,
    "/rest/v1.0/m-wallet/transfer/b2c/market",
    GlobalMerchantWrap<TransferB2CMarketReq>,
    TransferB2CMarketResp
);

post_form!(
    /// # 营销红包转账查询
    ///
    /// 商户通过请求该接口查询商户向用户转账的订单结果
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/get__rest__v1.0__m-wallet__transfer__b2c__query
    ///
    /// - body: [TransferB2CQueryReq]
    /// - return: [TransferB2CQueryResp]
    ///
    YeepayClient,
    transfer_b2c_query,
    "/rest/v1.0/m-wallet/transfer/b2c/query",
    GlobalMerchantWrap<TransferB2CQueryReq>,
    TransferB2CQueryResp
);

post_form!(
    /// # 开通免密支付
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__agreement__payment-request
    ///
    /// - body: [FreePaymentAgreementRequestReq]
    /// - return: [FreePaymentAgreementRequestResp]
    YeepayClient,
    free_payment_agreement_request,
    "/rest/v1.0/m-wallet/agreement/payment-request",
    GlobalMerchantWrap<FreePaymentAgreementRequestReq>,
    FreePaymentAgreementRequestResp
);

post_form!(
    /// # 免密支付协议查询
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__agreement__payment-query
    ///
    /// - body: [FreePaymentAgreementQueryReq]
    /// - return: [FreePaymentAgreementQueryResp]
    YeepayClient,
    free_payment_agreement_query,
    "/rest/v1.0/m-wallet/agreement/payment-query",
    GlobalMerchantWrap<FreePaymentAgreementQueryReq>,
    FreePaymentAgreementQueryResp
);

post_form!(
    /// # 收银台统一下单
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__cashier__unified__order
    ///
    /// - body: [OrderCreateReq]
    /// - return: [OrderCreateResp]
    YeepayClient,
    order_create,
    "/rest/v1.0/cashier/unified/order",
    GlobalMerchantWrap<OrderCreateReq>,
    OrderCreateResp
);

get!(
    /// # 查询订单
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/get__rest__v1.0__trade__order__query
    ///
    /// - body: [OrderQueryReq]
    /// - return: [OrderQueryResp]
    YeepayClient,
    order_query,
    "/rest/v1.0/trade/order/query",
    GlobalMerchantWrap<OrderQueryReq>,
    OrderQueryResp
);

post_form!(
    /// # 关闭订单
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__trade__order__close
    ///
    /// - body: [OrderCloseReq]
    /// - return: [OrderCloseResp]
    YeepayClient,
    order_close,
    "/rest/v1.0/trade/order/close",
    GlobalMerchantWrap<OrderCloseReq>,
    OrderCloseResp
);

post_form!(
    /// # 订单退款
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__trade__refund
    ///
    /// 使用说明
    /// - 1.订单状态为“SUCCESS”时才能发起退款。请勿频繁调用，两笔退款请求之间请间隔3s发起。
    /// - 2.退款支持单笔交易分多次退款，多次退款需要提交原支付订单的商户订单号和不同的商户退款请求号，累计退款金额不能超过原订单金额。
    /// - 3.退款支持幂等，一笔退款失败后重新提交，支持不更换商户退款请求号，使用相同的商户退款请求号再次请求退款。
    /// - 4.申请退款接口的响应参数code仅代表业务的受理情况，具体退款是否成功，需要通过申请退款或查询退款接口返回的status获取退款结果。
    ///     - （1）当响应参数code=OPR00000时,说明易宝已受理该笔退款,此时需要根据status来判断退款状态；
    ///     - （2）当响应参数code≠OPR00000时,说明易宝没有受理该笔退款，可根据返回的message进行相应处理。
    /// - 5.由于渠道对于退款有效期的限制，建议支付完成1年内操作退款。
    ///     - （1）聚合类交易（微信、支付宝、云闪付）：退款有效期365天。
    ///     - （2）网银类交易依赖银行退款有效期：工行B2B、农行90天；建行155天；其他银行180天。
    ///     - （3）快捷、刷卡类交易依赖银行退款有效期：一般是180天。
    ///
    ///
    /// - body: [OrderRefundReq]
    /// - return: [OrderRefundResp]
    YeepayClient,
    order_refund,
    "/rest/v1.0/trade/refund",
    GlobalMerchantWrap<OrderRefundReq>,
    OrderRefundResp
);

get!(
    /// # 订单退款查询
    ///
    /// 提交退款申请后，调用该接口查询退款状态
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/get__rest__v1.0__trade__refund__query
    ///
    /// - body: [OrderRefundQueryReq]
    /// - return: [OrderRefundQueryResp]
    YeepayClient,
    order_refund_query,
    "/rest/v1.0/trade/refund/query",
    GlobalMerchantWrap<OrderRefundQueryReq>,
    OrderRefundQueryResp
);

post_form!(
    /// # 申请订单分账
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__divide__apply
    ///
    /// - body: [OrderDivideApplyReq]
    /// - return: [OrderDivideApplyResp]
    YeepayClient,
    order_divide_apply,
    "/rest/v1.0/divide/apply",
    GlobalMerchantWrap<OrderDivideApplyReq>,
    OrderDivideApplyResp
);

get!(
    /// # 订单分账查询
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/get__rest__v1.0__divide__query
    ///
    /// - body: [OrderDivideQueryReq]
    /// - return: [OrderDivideQueryResp]
    YeepayClient,
    order_divide_query,
    "/rest/v1.0/divide/query",
    GlobalMerchantWrap<OrderDivideQueryReq>,
    OrderDivideQueryResp
);

post_form!(
    /// # 订单分账资金退回
    ///
    /// 平台商可调用此接口，将已分账的资金从分账接收方的账户回退给分账方账户。退款时如需分账接收方归还分账资金，可以先调此接口，再发起退款。
    ///
    /// 注意：分账订单的退款与分账资金归还并无强耦合，分账资金归还可先于退款发起，可后于退款发起，或者根据分账接收方与商户的约定，不发起分账资金归还。
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__divide__back
    ///
    /// - body: [OrderDivideBackReq]
    /// - return: [OrderDivideBackResp]
    YeepayClient,
    order_divide_back,
    "/rest/v1.0/divide/back",
    GlobalMerchantWrap<OrderDivideBackReq>,
    OrderDivideBackResp
);

get!(
    /// # 订单分账资金退回查询
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/get__rest__v1.0__divide__back__query
    ///
    /// - body: [OrderDivideBackQueryReq]
    /// - return: [OrderDivideBackQueryResp]
    YeepayClient,
    order_divide_back_query,
    "/rest/v1.0/divide/back/query",
    GlobalMerchantWrap<OrderDivideBackQueryReq>,
    OrderDivideBackQueryResp
);

post_form!(
    /// # 订单分账完成
    ///
    /// 订单后续不需要再进行分账，可直接调用此接口将订单剩余可分账金额全部给收款商户
    ///
    /// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__divide__complete
    ///
    /// - body: [OrderDivideCompleteReq]
    /// - return: [OrderDivideCompleteResp]
    YeepayClient,
    order_divide_complete,
    "/rest/v1.0/divide/complete",
    GlobalMerchantWrap<OrderDivideCompleteReq>,
    OrderDivideCompleteResp
);
