<?php

declare(strict_types=1);

final readonly class Box
{
    const N = 0;

    public function __construct(public int $n) {}

    public static function make(int $n): Box
    {
        return new Box($n);
    }

    public function size(): int
    {
        return $this->n;
    }

    public function parentOne(): int
    {
        return parent::N;
    }

    public function parentTwo(): Box
    {
        return new parent(1);
    }

    public function late(): int
    {
        return static::size();
    }

    final private function hidden(): int
    {
        return 1;
    }

    public function getHTTP(): int
    {
        return 1;
    }

    public function gethttp(): int
    {
        return 2;
    }
}

function through(Box $box): int
{
    return $box->make(1)->n;
}

function rest(int $n): int
{
    return intdiv($n, Box::N);
}

$box = Box::make(2);
$box::make(3);
echo through($box), "\n";
echo __NAMESPACE__, "\n";
