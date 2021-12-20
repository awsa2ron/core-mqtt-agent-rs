#[cfg(test)]
mod tests {
    use std::ffi::CString;
    #[test]
    // extern crate core_mqtt_sys as ffi;
    fn macth_topic() {
        let topic = "topic/match/1";
        let filter = "topic/#";
        let mut ret: bool = false;
        unsafe {
            let r = &mut ret as *mut bool;
            // let ret:usize = 0;
            let t = CString::new(topic).expect("CString::new failed");
            let tlen: u16 = topic.len().try_into().unwrap();
            let f = CString::new(filter).expect("CString::new failed");
            let flen: u16 = filter.len().try_into().unwrap();
            let status: ffi::MQTTConnectionStatus =
                ffi::MQTT_MatchTopic(t.as_ptr(), tlen, f.as_ptr(), flen, r);
            // println!(
            //     "topic:{}, filter:{}, status:{}, match?{}",
            //     topic, filter, status, ret
            // );
        }
        assert_eq!(ret, true);
    }
}
