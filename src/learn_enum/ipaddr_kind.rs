pub enum IpaddrKind {
    V4,
    V6,
}

// 枚举变体中可以嵌入任意数据类型
pub enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}



pub fn route(ip_kind: IpaddrKind) {

}