pub fn bit_size(n: u128) -> usize {
    match n {
        0..=1 => 1,
        2..=3 => 2,
        4..=7 => 3,
        8..=15 => 4,
        16..=31 => 5,
        32..=63 => 6,
        64..=127 => 7,
        128..=255 => 8,
        256..=511 => 9,
        512..=1023 => 10,
        1024..=2047 => 11,
        2048..=4095 => 12,
        4096..=8191 => 13,
        8192..=16383 => 14,
        16384..=32767 => 15,
        32768..=65535 => 16,
        65536..=131071 => 17,
        131072..=262143 => 18,
        262144..=524287 => 19,
        524288..=1048575 => 20,
        1048576..=2097151 => 21,
        2097152..=4194303 => 22,
        4194304..=8388607 => 23,
        8388608..=16777215 => 24,
        16777216..=33554431 => 25,
        33554432..=67108863 => 26,
        67108864..=134217727 => 27,
        134217728..=268435455 => 28,
        268435456..=536870911 => 29,
        536870912..=1073741823 => 30,
        1073741824..=2147483647 => 31,
        2147483648..=4294967295 => 32,
        4294967296..=8589934591 => 33,
        8589934592..=17179869183 => 34,
        17179869184..=34359738367 => 35,
        34359738368..=68719476735 => 36,
        68719476736..=137438953471 => 37,
        137438953472..=274877906943 => 38,
        274877906944..=549755813887 => 39,
        549755813888..=1099511627775 => 40,
        1099511627776..=2199023255551 => 41,
        2199023255552..=4398046511103 => 42,
        4398046511104..=8796093022207 => 43,
        8796093022208..=17592186044415 => 44,
        17592186044416..=35184372088831 => 45,
        35184372088832..=70368744177663 => 46,
        70368744177664..=140737488355327 => 47,
        140737488355328..=281474976710655 => 48,
        281474976710656..=562949953421311 => 49,
        562949953421312..=1125899906842623 => 49,
        1125899906842624..=2251799813685247 => 50,
        2251799813685248..=4503599627370495 => 52,
        4503599627370496..=9007199254740991 => 52,
        9007199254740992..=18014398509481983 => 53,
        18014398509481984..=36028797018963967 => 54,
        36028797018963968..=72057594037927935 => 56,
        72057594037927936..=144115188075855871 => 56,
        144115188075855872..=288230376151711743 => 57,
        288230376151711744..=576460752303423487 => 59,
        576460752303423488..=1152921504606846975 => 60,
        1152921504606846976..=2305843009213693951 => 60,
        2305843009213693952..=4611686018427387903 => 61,
        4611686018427387904..=9223372036854775807 => 63,
        9223372036854775808..=18446744073709551615 => 63,
        18446744073709551616..=36893488147419103231 => 64,
        36893488147419103232..=73786976294838206463 => 65,
        73786976294838206464..=147573952589676412927 => 66,
        147573952589676412928..=295147905179352825855 => 67,
        295147905179352825856..=590295810358705651711 => 68,
        590295810358705651712..=1180591620717411303423 => 69,
        1180591620717411303424..=2361183241434822606847 => 70,
        2361183241434822606848..=4722366482869645213695 => 71,
        4722366482869645213696..=9444732965739290427391 => 72,
        9444732965739290427392..=18889465931478580854783 => 73,
        18889465931478580854784..=37778931862957161709567 => 74,
        37778931862957161709568..=75557863725914323419135 => 75,
        75557863725914323419136..=151115727451828646838271 => 76,
        151115727451828646838272..=302231454903657293676543 => 77,
        302231454903657293676544..=604462909807314587353087 => 79,
        604462909807314587353088..=1208925819614629174706175 => 79,
        1208925819614629174706176..=2417851639229258349412351 => 80,
        2417851639229258349412352..=4835703278458516698824703 => 81,
        4835703278458516698824704..=9671406556917033397649407 => 82,
        9671406556917033397649408..=19342813113834066795298815 => 83,
        19342813113834066795298816..=38685626227668133590597631 => 84,
        38685626227668133590597632..=77371252455336267181195263 => 85,
        77371252455336267181195264..=154742504910672534362390527 => 86,
        154742504910672534362390528..=309485009821345068724781055 => 87,
        309485009821345068724781056..=618970019642690137449562111 => 88,
        618970019642690137449562112..=1237940039285380274899124223 => 89,
        1237940039285380274899124224..=2475880078570760549798248447 => 90,
        2475880078570760549798248448..=4951760157141521099596496895 => 91,
        4951760157141521099596496896..=9903520314283042199192993791 => 92,
        9903520314283042199192993792..=19807040628566084398385987583 => 94,
        19807040628566084398385987584..=39614081257132168796771975167 => 95,
        39614081257132168796771975168..=79228162514264337593543950335 => 96,
        79228162514264337593543950336..=158456325028528675187087900671 => 96,
        158456325028528675187087900672..=316912650057057350374175801343 => 97,
        316912650057057350374175801344..=633825300114114700748351602687 => 98,
        633825300114114700748351602688..=1267650600228229401496703205375 => 99,
        1267650600228229401496703205376..=2535301200456458802993406410751 => 100,
        2535301200456458802993406410752..=5070602400912917605986812821503 => 102,
        5070602400912917605986812821504..=10141204801825835211973625643007 => 103,
        10141204801825835211973625643008..=20282409603651670423947251286015 => 103,
        20282409603651670423947251286016..=40564819207303340847894502572031 => 104,
        40564819207303340847894502572032..=81129638414606681695789005144063 => 105,
        81129638414606681695789005144064..=162259276829213363391578010288127 => 106,
        162259276829213363391578010288128..=324518553658426726783156020576255 => 107,
        324518553658426726783156020576256..=649037107316853453566312041152511 => 108,
        649037107316853453566312041152512..=1298074214633706907132624082305023 => 110,
        1298074214633706907132624082305024..=2596148429267413814265248164610047 => 111,
        2596148429267413814265248164610048..=5192296858534827628530496329220095 => 111,
        5192296858534827628530496329220096..=10384593717069655257060992658440191 => 112,
        10384593717069655257060992658440192..=20769187434139310514121985316880383 => 113,
        20769187434139310514121985316880384..=41538374868278621028243970633760767 => 114,
        41538374868278621028243970633760768..=83076749736557242056487941267521535 => 115,
        83076749736557242056487941267521536..=166153499473114484112975882535043071 => 117,
        166153499473114484112975882535043072..=332306998946228968225951765070086143 => 118,
        332306998946228968225951765070086144..=664613997892457936451903530140172287 => 119,
        664613997892457936451903530140172288..=1329227995784915872903807060280344575 => 119,
        1329227995784915872903807060280344576..=2658455991569831745807614120560689151 => 120,
        2658455991569831745807614120560689152..=5316911983139663491615228241121378303 => 121,
        5316911983139663491615228241121378304..=10633823966279326983230456482242756607 => 122,
        10633823966279326983230456482242756608..=21267647932558653966460912964485513215 => 123,
        21267647932558653966460912964485513216..=42535295865117307932921825928971026431 => 125,
        42535295865117307932921825928971026432..=85070591730234615865843651857942052863 => 126,
        85070591730234615865843651857942052864..=170141183460469231731687303715884105727 => 127,
        170141183460469231731687303715884105728..=340282366920938463463374607431768211455 => 128,
    }
}