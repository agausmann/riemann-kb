include <vars.scad>

module m2(position)
{
    translate(position) circle(d = 2.2 * mm);
}

module m2_insert(position)
{
    translate(position) circle(d = 3 * mm);
}

module m2_3dp(position)
{
    // Additional tolerance over typical M2 holes
    translate(position) circle(d = 2.4 * mm);
}

module m2_head(position)
{
    translate(position) circle(d = 4.2 * mm);
}

module m2_spacer(position)
{
    translate(position) circle(d = 3.6 * mm);
}

module mx_1u()
{
    square(14 * mm, center = true);
}

module skuf_slot()
{
    hull()
    {
        for (i = [ -0.5, 0.5 ])
        {
            translate([ i * (skuf_width - skuf_height), 0 ]) circle(d = skuf_height);
        }
    }
}

module dovetail(width, height, skew)
{
    polygon([
        [ -0.5 * width, -0.5 * (height + skew) ],
        [ 0.5 * width, -0.5 * (height - skew) ],
        [ 0.5 * width, 0.5 * (height - skew) ],
        [ -0.5 * width, 0.5 * (height + skew) ],
    ]);
}
