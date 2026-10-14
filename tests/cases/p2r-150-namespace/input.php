<?php

declare(strict_types=1);

namespace Acme\Shop;

use Acme\Shop\Size as Sz;
use Acme\Shop\{Box, Size};
use function Acme\Shop\describe;
use InvalidArgumentException;

enum Size: string
{
    case Small = 's';
    case Large = 'l';
}

final readonly class Box
{
    public function __construct(public Size $size, public int $count) {}
}

function describe(Box $box): string
{
    return $box->size->value . $box->count;
}

echo describe(new Box(Sz::Large, 3)), "\n";
echo describe(new \Acme\Shop\Box(Size::Small, 1)), "\n";
