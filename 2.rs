use std::process::Command;




fn main() {
    println!("welcome back vynxidn");

    let banner = "
    #     # #     # #     # #     # ### ######  #     #    ######  ######  #######       # #######  #####  ####### 
    #     #  #   #  ##    #  #   #   #  #     # ##    #    #     # #     # #     #       # #       #     #    #    
    #     #   # #   # #   #   # #    #  #     # # #   #    #     # #     # #     #       # #       #          #    
    #     #    #    #  #  #    #     #  #     # #  #  #    ######  ######  #     #       # #####   #          #    
     #   #     #    #   # #   # #    #  #     # #   # #    #       #   #   #     # #     # #       #          #    
      # #      #    #    ##  #   #   #  #     # #    ##    #       #    #  #     # #     # #       #     #    #    
       #       #    #     # #     # ### ######  #     #    #       #     # #######  #####  #######  #####     #    
                                                                                                              
    ";
    println!("{}", banner);
    let uname = Command::new("uname")
    .arg("-a")
    .output()
    .expect("gagal menjalan uname -a ");
    println!("nama sysytem: {}", String::from_utf8_lossy(&uname.stdout));

    let code1rs = Command::new("./1")
    .output()
    .expect("gagal menjalankan ./1");
    println!("{}", String::from_utf8_lossy(&code1rs.stdout))

}