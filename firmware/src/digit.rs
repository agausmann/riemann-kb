#![allow(dead_code)]

/*
Template:

digit!(
      .
    .   .
      .
    .   .
      .    .
)
*/
macro_rules! digit {
    (@bit .) => {
        0
    };
    (@bit 0) => {
        0
    };
    (@bit $x:tt) => {
        1
    };
    (
           $a:tt
        $f:tt $b:tt
           $g:tt
        $e:tt $c:tt
           $d:tt $dp:tt
    ) => {
        (digit!(@bit $a) << 0)
        | (digit!(@bit $b) << 1)
        | (digit!(@bit $c) << 2)
        | (digit!(@bit $d) << 3)
        | (digit!(@bit $e) << 4)
        | (digit!(@bit $f) << 5)
        | (digit!(@bit $g) << 6)
        | (digit!(@bit $dp) << 7)
    };
}

/*
Template:

double_digit!(
      .         .
    .   .     .   .
      .         .
    .   .     .   .
      .    .    .    .
)
*/
macro_rules! double_digit {
    (
        $a1:tt           $a2:tt
     $f1:tt $b1:tt    $f2:tt $b2:tt
        $g1:tt           $g2:tt
     $e1:tt $c1:tt    $e2:tt $c2:tt
        $d1:tt $dp1:tt   $d2:tt $dp2:tt
    ) => {
        (
            digit!(
                $a1
                $f1 $b1
                $g1
                $e1 $c1
                $d1 $dp1
            ) << 0
        ) | (
            digit!(
                $a2
              $f2 $b2
                $g2
              $e2 $c2
                $d2 $dp2
            ) << 8
        )
    }
}

pub const DIGITS: [u16; 10] = [
    digit!(
        1
      1   1
        .
      1   1
        1    .
    ),
    digit!(
        .
      .   1
        .
      .   1
        .    .
    ),
    digit!(
        1
      .   1
        1
      1   .
        1    .
    ),
    digit!(
        1
      .   1
        1
      .   1
        1    .
    ),
    digit!(
        .
      1   1
        1
      .   1
        .    .
    ),
    digit!(
        1
      1   .
        1
      .   1
        1    .
    ),
    digit!(
        1
      1   .
        1
      1   1
        1    .
    ),
    digit!(
        1
      .   1
        .
      .   1
        .    .
    ),
    digit!(
        1
      1   1
        1
      1   1
        1    .
    ),
    digit!(
        1
      1   1
        1
      .   1
        1    .
    ),
];

pub const CAPS: u16 = double_digit!(
      1         1
    1   .     1   1
      .         1
    1   .     1   1
      1    .    .    .
);

pub const FU: u16 = double_digit!(
      1         .
    1   .     1   1
      1         .
    1   .     1   1
      .    .    1    .
);

pub const META: u16 = double_digit!(
        1         1
      1   .     1   1
        1         1
      1   .     1   1
        1    .    .    .
);

pub const CIRCLES: &[u16] = &[
    double_digit!(
        1         1
      .   .     .   .
        .         .
      .   .     .   .
        .    .    .    .
    ),
    double_digit!(
        .         .
      .   1     .   1
        .         .
      .   .     .   .
        .    .    .    .
    ),
    double_digit!(
        .         .
      .   .     .   .
        .         .
      .   1     .   1
        .    .    .    .
    ),
    double_digit!(
          .         .
        .   .     .   .
          .         .
        .   .     .   .
          1    .    1    .
    ),
    double_digit!(
          .         .
        .   .     .   .
          .         .
        1   .     1   .
          .    .    .    .
    ),
    double_digit!(
          .         .
        1   .     1   .
          .         .
        .   .     .   .
          .    .    .    .
    ),
];

pub const MIRROR_CIRCLES: &[u16] = &[
    double_digit!(
        1         1
      .   .     .   .
        .         .
      .   .     .   .
        .    .    .    .
    ),
    double_digit!(
          .         .
        .   1     1   .
          .         .
        .   .     .   .
          .    .    .    .
    ),
    double_digit!(
          .         .
        .   .     .   .
          .         .
        .   1     1   .
          .    .    .    .
    ),
    double_digit!(
          .         .
        .   .     .   .
          .         .
        .   .     .   .
          1    .    1    .
    ),
    double_digit!(
          .         .
        .   .     .   .
          .         .
        1   .     .   1
          .    .    .    .
    ),
    double_digit!(
          .         .
        1   .     .   1
          .         .
        .   .     .   .
          .    .    .    .
    ),
];

pub const WIPER: &[u16] = &[
    double_digit!(
          .         .
        1   .     .   .
          .         .
        1   .     .   .
          .    .    .    .
    ),
    double_digit!(
          .         .
        .   1     .   .
          .         .
        .   1     .   .
          .    .    .    .
    ),
    double_digit!(
          .         .
        .   .     1   .
          .         .
        .   .     1   .
          .    .    .    .
    ),
    double_digit!(
          .         .
        .   .     .   1
          .         .
        .   .     .   1
          .    .    .    .
    ),
    double_digit!(
          .         .
        .   .     1   .
          .         .
        .   .     1   .
          .    .    .    .
    ),
    double_digit!(
          .         .
        .   1     .   .
          .         .
        .   1     .   .
          .    .    .    .
    ),
];
