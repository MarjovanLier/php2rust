<?php

declare(strict_types=1);

enum Coin: string
{
    case Heads = 'h';
    case Tails = 't';

    public static function guess(string $side): ?self
    {
        if ($side === 'h') {
            return self::Heads;
        }
        return null;
    }
}

final readonly class Point
{
    public function __construct(public int $x, public int $y) {}

    public static function origin(): static
    {
        return new static(0, 0);
    }

    public function moved(int $by): self
    {
        return new self($this->x + $by, $this->y - $by);
    }

    public function same(self $other): bool
    {
        return $this->x === $other->x && $this->y === $other->y;
    }
}

$coin = Coin::guess('h');
if ($coin !== null) {
    echo $coin->value, "\n";
}
if (Coin::guess('t') === null) {
    echo "none\n";
}
$point = Point::origin()->moved(3);
echo $point->x, ' ', $point->y, "\n";
if (!$point->same(Point::origin())) {
    echo "moved\n";
}
