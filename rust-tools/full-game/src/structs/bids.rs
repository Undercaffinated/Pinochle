pub enum Bid {
    Open,
    OpenOnce,
    Pass,
    WontHurt,
    ChooseWisely,
    Fill,
    Bid(u8),
    BidOnce(u8),
    Undeclared,
}