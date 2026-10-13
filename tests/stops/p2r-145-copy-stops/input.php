<?php

declare(strict_types=1);

final class Box
{
    public int $n = 0;
}

final class Holder
{
    public function __construct(public Box $box)
    {
    }

    public function me(): Holder
    {
        return $this;
    }

    public function inner(): Box
    {
        return $this->box;
    }

    public function put(Box $box): void
    {
        $this->box = $box;
    }
}

function readAgain(): void
{
    $a = new Box();
    $b = $a;
    echo $a->n + $b->n, "\n";
}

/**
 * @param list<Box> $boxes
 */
function element(array $boxes): Box
{
    $x = $boxes[0];
    return $x;
}

function putThenRead(Holder $holder): void
{
    $a = new Box();
    $holder->put($a);
    echo $a->n, "\n";
}

function keep(Box $x, Box $y): Holder
{
    echo $y->n, "\n";
    return new Holder($x);
}

function twice(): void
{
    $a = new Box();
    keep($a, $a);
}
