<?php

declare(strict_types=1);

namespace Shop\Store;

use Shop\Model\Size;

enum Shape
{
    case Box;
    case Tube;
}

final class Box
{
    public int $total = 0;

    public function total(): int
    {
        return $this->total;
    }

    public function fill(Size $size = Size::Small): void
    {
        $this->total += 1;
    }

    public function shape(Shape $shape): string
    {
        return match ($shape) {
            Shape::Box => 'box',
            Shape::Tube => 'tube',
            default => 'other',
        };
    }
}

final class Crate
{
    public static function make(): Box
    {
        $box = new Box();
        $box->fill();
        return $box;
    }
}
