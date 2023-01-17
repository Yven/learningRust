#![allow(unused)]

use core::time;
use chrono::Utc;
use num::ToPrimitive;


// 资源
#[derive(Debug)]
struct ResourceReport {
    aid: i32,
    name: String,
    desc: String,
    time: TimeInterval,
    audit: ResourceAudit,
}

// 资源审核状态
#[derive(Debug)]
struct ResourceAudit {
    status: AuditStatus,
    reason: String,
    time: i64,
}

// 资源持续时间
#[derive(Debug)]
struct TimeInterval {
    start: i64,
    end: i64,
}

// 审核状态枚举
#[derive(Debug,Clone,Copy)]
enum AuditStatus {
    Pass,
    ContentWrong,
    Unclear,
    Other,
    NoAudit,
}

impl ResourceReport {
    // 创建资源
    fn new(aid: i32, name: String, desc: String, dur_time: i32) -> ResourceReport {
        ResourceReport {
            aid,
            name,
            desc,
            time: TimeInterval::new(dur_time),
            audit: ResourceAudit::new(),
        }
    }

    // 审核资源
    fn audit(&mut self, status: &AuditStatus, reason: &String) -> () {
        self.audit.change(status, reason);
    }
}

impl TimeInterval {
    // 根据时长创建时间区间
    fn new(dur_time: i32) -> TimeInterval {
        TimeInterval { 
            start: Utc::now().timestamp(),
            end: Utc::now().timestamp()+dur_time.to_i64().unwrap(),
        }
    }
}

impl ResourceAudit {
    // 创建默认审核状态
    fn new() -> ResourceAudit{
        ResourceAudit {
            status: AuditStatus::NoAudit,
            reason: "".to_string(),
            time: Utc::now().timestamp(),
        }
    }

    // 修改审核状态
    fn change(&mut self, status: &AuditStatus, reason: &String) -> () {
        self.status = *status;
        self.reason = reason.to_string();
        self.time = Utc::now().timestamp();
    }

    // 获取审核不通过原因
    fn reason(&self) -> String {
        self.reason.to_string()
    }
}

impl AuditStatus {
    // 获取不同审核类型对应的名称
    fn name(&self) -> String {
        let name = match self {
            AuditStatus::Pass => "已通过",
            AuditStatus::ContentWrong => "内容错误",
            AuditStatus::Unclear => "描述不清晰",
            AuditStatus::Other => "其他原因",
            AuditStatus::NoAudit => "未审核",
        };
        name.to_string()
    }
}

fn main() {
    let mut resource = ResourceReport::new(7, "测试".to_string(), "这里是描述".to_string(), 30);
    println!("{:#?}", resource);
    println!("审核状态：{}, 原因：{}", resource.audit.status.name(), resource.audit.reason());
    resource.audit(&AuditStatus::ContentWrong, &"测试不通过".to_string());
    println!("审核状态：{}, 原因：{}", resource.audit.status.name(), resource.audit.reason());
    println!("{:#?}", resource);
}