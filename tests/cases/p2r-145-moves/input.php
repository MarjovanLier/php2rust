<?php

declare(strict_types=1);

final class Box
{
    public int $n = 0;
}

final class Shelf
{
    /** @var list<Box> */
    public array $boxes = [];

    public function put(Box $box): void
    {
        $this->boxes[] = $box;
    }
}

function wrap(Box $box): Box
{
    $box->n++;
    return $box;
}

$a = new Box();
$b = $a;
$b->n = 2;
$c = wrap($b);
$shelf = new Shelf();
$shelf->put($c);
echo count($shelf->boxes), " ", $shelf->boxes[0]->n, "\n";
