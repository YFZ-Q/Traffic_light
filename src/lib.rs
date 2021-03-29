//定义trait
pub trait TrafficInfo {
    fn time(traffic:TrafficLight) -> u8;
}

//红绿灯的枚举
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

//为TrafficLight类型实现trait
impl TrafficInfo for TrafficLight {
    fn time(traffic:TrafficLight) -> u8 {
        //匹配操作
        match traffic {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 20
        }
    }
}