<?php

declare(strict_types=1);

final readonly class Point
{
    public int $y;

    public function __construct(public int $x) {}
}

final class Size
{
    public function __construct(int $width) {}
}

final class Name
{
    public function __construct(
        public string $first {
            set => trim($value);
        },
    ) {}
}

$p = new Point(1);
echo $p->x, "\n";
