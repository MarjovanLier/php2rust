<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Widget extends Model
{
}

final class Gadget extends Model
{
}

final class Sprocket extends Model
{
}

final class Lever extends Model
{
}

final class Pulley extends Model
{
}

final class Gear extends Model
{
}

final class Bolt extends Model
{
}

final class Nut extends Model
{
}

final class Washer extends Model
{
}

final class Spring extends Model
{
}

function springName(Spring $spring): string
{
    return $spring->name;
}
