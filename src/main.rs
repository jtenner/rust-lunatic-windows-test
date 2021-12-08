mod libtelnet;
use core::ffi::{ c_void };

//  struct telnet_telopt_t {
//  	short telopt;      /*!< one of the TELOPT codes or -1 */
//  	unsigned char us;  /*!< TELNET_WILL or TELNET_WONT */
//  	unsigned char him; /*!< TELNET_DO or TELNET_DONT */
//  };


fn main() {
    let a = unsafe { libtelnet::telnet_init(&[] as *const libtelnet::telnet_telopt_t, None, 0, 0 as *mut c_void) };
    unsafe { libtelnet::telnet_free(a) };
}
