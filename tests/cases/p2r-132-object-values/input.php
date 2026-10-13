<?php

declare(strict_types=1);

readonly class Item
{
    public function __construct(public string $name, public int $price)
    {
    }

    public function itself(): Item
    {
        return $this;
    }

    public function describe(): string
    {
        return label($this);
    }
}

function label(Item $item): string
{
    return $item->name . '=' . $item->price;
}

function keep(Item $item): Item
{
    return $item;
}

/** @param list<Item> $items */
function total(array $items): int
{
    $sum = 0;
    foreach ($items as $item) {
        $sum += $item->price;
    }
    return $sum;
}

$pen = new Item('pen', 3);
$copy = $pen;
$items = [$pen, keep($copy)];
$items[] = $pen->itself();
echo total($items), ' ', count($items), "\n";
echo $copy->describe(), "\n";
foreach ($items as $item) {
    echo $item->name, "\n";
}
