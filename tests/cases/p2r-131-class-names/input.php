<?php

declare(strict_types=1);

readonly class order_line
{
    public function __construct(public string $productName, public int $type)
    {
    }

    public function getLabel(): string
    {
        return $this->productName . '#' . $this->type;
    }
}

function make(string $name): ORDER_LINE
{
    return new Order_Line($name, 2);
}

$line = make('pen');
echo $line->GETLABEL(), "\n";
echo $line->getlabel(), ' ', $line->productName, "\n";
