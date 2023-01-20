#![allow(unused)]
//! 这里是餐厅包相关注释

pub mod backend_house;

// 引入再导出此包
pub use crate::backend_house::ketchen;

/// 餐厅前台模块
/// 
/// 主要包括
/// 1. `host`客户相关行为
/// 2. `serving`服务员相关行为
#[doc(alias("front","house"))]
pub mod front_house {
    /// host客户模块
    /// 
    /// 1. `add_waitlist()` 将食物添加到候选栏
    /// 2. `seat_table()`   找个位置坐下
    pub mod host {
        /// 将食物添加到候选栏
        /// 
        /// [`Option`]，[`seat_table`]，[`crate::backend_house::ketchen`]
        /// 
        /// # Exmaples
        /// ```
        /// let customer = world_hello::front_house::host::add_waitlist();
        /// assert_eq!(customer, ());
        /// ```
        pub fn add_waitlist() {}

        /// 坐下
        /// 
        /// # Exmaples
        /// 
        /// ```
        /// println!("test open");
        /// # let hidden_table = world_hello::front_house::host::seat_table();
        /// # assert_eq!(hidden_table, ());
        /// ```
        pub fn seat_table() {}
    }

    pub mod serving {
        // 在父级模块中可见
        pub(super) fn take_order() {
            // 使用父级下的包，父级对子集完全开放，但子集只对父级开放pub
            super::host::add_waitlist();
            // 同级下的方法，可以不加self
            self::serve_order();
        }
        // 仅在当前模块中可见
        pub(self) fn serve_order() {}
        // 在当前包中可见
        pub(crate) fn soliciting() {}
        // 在指定包中可见，该包只能是父模块或祖先模块
        pub(in crate::front_house) fn good_bye() {}
        // 不加pub则默认为私有
        fn take_payment() {}
    }
}

// 结构体默认为私有
pub struct Menu {
    pub name: String,
    pub price: f32,
    pub desc: String,
    cost: f32,
    cooker: String,
}

// 枚举默认为公有
enum OrderStatus {
    WaitForPay,
    Eatting,
    Settle,
    Finished,
}

fn eat_to_restaurant() {
    // 使用相对路径访问
    front_house::host::seat_table();
    // 使用绝对路径访问
    crate::front_house::host::add_waitlist();

    ketchen::cook();
}
