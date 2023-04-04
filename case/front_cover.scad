include <vars.scad>

module front_cover()
{
    // Just fill the space
    cube([ front_cover_width, front_cover_depth, front_cover_height ]);
}

front_cover();
