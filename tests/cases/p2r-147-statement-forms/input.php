<?php

declare(strict_types=1);

final class Acc
{
    public int $qty = 0;
    public string $name = 'acc';

    public function next(): int
    {
        $this->qty += 1;
        return $this->qty;
    }

    public function total(): int
    {
        return $this->qty * 2;
    }

    public function label(): string
    {
        return $this->name . "!";
    }

    public function add(int $n): void
    {
        $this->qty += $n;
    }

    public function setName(string $name): void
    {
        $this->name = $name;
    }

    public function tag(): string
    {
        $this->qty++;
        return "t";
    }

    public function grow(): void
    {
        $this->qty += $this->next();
        $this->qty = $this->next() * $this->qty;
        $this->name = $this->label() . $this->tag();
        bump($this, $this->qty);
    }
}

function bump(Acc $acc, int $by): void
{
    $acc->qty += $by;
}

function twice(Acc $acc): void
{
    bump($acc, $acc->qty);
}

$a = new Acc();
$a->add($a->total());
$a->setName($a->label());
$a->grow();
twice($a);
echo $a->qty, " ", $a->name, "\n";
