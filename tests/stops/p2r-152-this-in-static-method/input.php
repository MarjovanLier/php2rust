<?php

declare(strict_types=1);

// PHP throws an Error when the static method reads $this.
final readonly class Point
{
    public function __construct(public int $x) {}

    public static function origin(): self
    {
        return $this;
    }
}

echo Point::origin()->x, "\n";
