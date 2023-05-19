use img2ascii::{resize_image, image2ascii};

fn main(){

    let original_image_path = "/home/egg/Downloads/IMG-7249-removebg-preview(1).png";
    let resized_image_path = "i.png";
    
    resize_image(original_image_path, resized_image_path, 20.0);

    let ascii_scale = " .'`,^:\";~-_+<>i!lI?/\\|()1{}[]rcvunxzjftLCJUYXZO0Qoahkbdpqwm*WMB8&%$#@";
    image2ascii("i.png", "1.egg",ascii_scale);
    
    //let ascii_scale2 = " `´¨·…¸ˆ˜’‚‘’:;›‹¹­°–²º³~“”„ª^—¯¦÷¡!|¬•«»/\\)(+{*}×?¿[†7i><ìï=íl1vjrîcotI¤%ƒJz‡¼½‰u¢sòön@CóÌÏ3±LÍV0£Y©wçù2™üa&úô5õ4f¾ÎxyOµ6eTUž9S®ŸkÝ§ûhàÇšäPZáñDF¥bÒÿåÖÞÓèýÙdë$ÜÚ8éðø¶Gâãmpß€AÔÕŠÛqêÐgXœŽRKHÀÄEþÁN#æÅBQWÂÃÈËÉMØŒÊÑÆ";
    //image2ascii("i.png", "2.egg",ascii_scale2);
    //let ascii_scale3 = " ▁▂▃▄▅▆▇█";
    //image2ascii("i.png", "3.egg",ascii_scale3);
    //let ascii_scale4 = " ▏▎▍▌▋▊";
    //image2ascii("i.png", "4.egg",ascii_scale4);
    //let ascii_scale5 = " ░▒▓█";
    //image2ascii("i.png", "5.egg",ascii_scale5);

    //let ascii_scale6 = " .:-=+*#%@";
    //image2ascii("i.png", "6.egg",ascii_scale6);

    //let ascii_scale7 = "🌑🌒🌓🌔🌕";
    //image2ascii("i.png", "7.egg",ascii_scale7);
}