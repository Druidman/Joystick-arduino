use serialport::*;
use evdev::{
    uinput:: {VirtualDevice},
    UinputAbsSetup, AbsInfo, AbsoluteAxisCode, InputEvent, EventType, KeyCode, AttributeSet
};
use std::{
    io:: {stdin, stdout, Write},
    thread,
    sync::mpsc,
    time::Duration
    

};

struct Axis {
    code: AbsoluteAxisCode,
    range: Vec<i32>,
}

const ARDUINO_PORT_NAME: &str = "/dev/ttyUSB0"; 

const BUTTONS: [KeyCode; 1] = [
    KeyCode::BTN_SOUTH
];

fn main(){
    let mut port: Box<dyn SerialPort> = serialport::new(ARDUINO_PORT_NAME,9600)
        .open()
        .expect("No such device");

    thread::sleep(Duration::from_secs(2));

    let axes: [Axis; 2] = setup_axes(&mut port);

    let mut device = VirtualDevice::builder()
        .expect("device creation went wrong")
        .name("VirtualJoystick");

    
    for axis in &axes {
        let abs_axis_info = AbsInfo::new(0, axis.range[0], axis.range[1], 0, 0, 1);
        let abs_axis = UinputAbsSetup::new(axis.code,abs_axis_info);
        device = device
            .with_absolute_axis(&abs_axis)
            .expect("axis insertion error");
    }

    let mut keys = AttributeSet::<KeyCode>::new();
    for button_code in BUTTONS {
        keys.insert(button_code);
    }
    device = device
        .with_keys(&keys)
        .expect("buttons/keys insertion error");

    let device = device
        .build()
        .expect("VirtualDevice build failed");

    let (sender, receiver) = mpsc::channel();
    thread::spawn(|| emit_device(device,receiver));
    
    loop {
        let values_str: Vec<String> = read_port_data(&mut port);
    
        let values: Vec<i32> = vec![
            values_str[0]
            .parse()
            .expect("parsing axis value went wrong"),
            values_str[1]
            .parse()
            .expect("parsing axis value went wrong")
        ];

        let axis_event_type = EventType::ABSOLUTE.0;
        for (value,axis) in values.iter().zip(axes.iter()){
            
            let input_event = InputEvent::new(axis_event_type, axis.code.0, *value);
            let _ = sender.send(input_event)
                .expect("send went wrong");
        }
        let input_event = InputEvent::new(EventType::KEY.0,KeyCode::BTN_SOUTH.0,1);
        let _ = sender.send(input_event)
            .expect("send went wrong");
    

        
        
   
    }
}

fn emit_device(mut device: VirtualDevice, receiver: mpsc::Receiver<InputEvent>){
    loop {
        let Ok(input_events) = receiver.recv() else {
            break ;
        };
        device.emit(&[input_events]).expect("emit went wrong");
        println!("inputs added: {:?}",input_events)
    }
    
}

fn setup_axes(port: &mut Box<dyn SerialPort>) -> [Axis; 2]{
    println!("Joystick setup process began");

    println!("Move X axis full right (roll right)");
    let _ = read_input_line();
 
    let roll_right_limit: i32 = read_port_data(port)[0]
       .parse()
       .expect("bad parse");
    println!("{:?}", roll_right_limit);

    println!("Move X axis full left (roll left)");
    let _ = read_input_line();
    let roll_left_limit: i32 = read_port_data(port)[0]
        .parse()
        .expect("bad parse");
    println!("{:?}",roll_left_limit);

    
    println!("Move Y axis full forward (pitch down)");
    let _ = read_input_line();
    let pitch_down_limit = read_port_data(port)[1]
        .parse()
        .expect("bad parse");
    println!("{:?}", pitch_down_limit);

    println!("Move Y axis full backward (pitch up)");
    let _ = read_input_line();
    let pitch_up_limit = read_port_data(port)[1]
        .parse()
        .expect("bad parse");
    println!("{:?}", pitch_up_limit);
    println!("Setup finished!");

    return [
        Axis {
  
            code: AbsoluteAxisCode::ABS_X,
            range: vec![roll_left_limit,roll_right_limit]
            
        },
        Axis {
 
            code: AbsoluteAxisCode::ABS_Y,
            range: vec![pitch_up_limit,pitch_down_limit]
        }
    ];
    


}

fn read_input_line() -> String{
    let mut buf = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut buf).expect("error reading line");

    buf
}

fn read_port_data(port: &mut Box<dyn SerialPort>) -> Vec<String>{
    let msg_to_write = b"SEND/";
    let _ = port.write(msg_to_write);

    let mut received_message = String::new();
    while received_message == ""{

        let _ = port.read_to_string(&mut received_message);
    }
    println!("{:?}",received_message);
    received_message.split(' ').map(|s| s.to_string()).collect()
 

}