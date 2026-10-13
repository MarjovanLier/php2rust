<?php

declare(strict_types=1);

final class Part
{
    public int $n = 0;
}

final class Acc
{
    public int $qty = 0;
    public string $text = 'a';
    /** @var list<Part> */
    public array $items = [];

    public function __construct(public Part $part)
    {
    }

    public function next(): int
    {
        $this->qty += 1;
        return $this->qty;
    }

    public function add(int $n): void
    {
        $this->qty += $n;
    }

    public function make(): Part
    {
        $this->qty += 1;
        return new Part();
    }

    public function bumpText(): string
    {
        $this->text .= "x";
        return $this->text;
    }

    public function copyTo(Acc $other): void
    {
        $other->qty = $this->qty;
    }

    public function appendMade(): void
    {
        $this->items[] = $this->make();
    }

    public function doubleText(): void
    {
        $this->text .= $this->text;
    }
}

function bump(Acc $acc, int $by): void
{
    $acc->qty += $by;
}

function pair(Part $part, Acc $acc): void
{
    $acc->qty = $part->n;
}

function local(): void
{
    $a = new Acc(new Part());
    bump($a, $a->qty);
}

function addNext(Acc $a): void
{
    $a->add($a->next());
}

function joinBumped(Acc $a): string
{
    return $a->text . $a->bumpText();
}

function toItself(Acc $a): void
{
    $a->copyTo($a);
}

function partFirst(Acc $a): void
{
    pair($a->part, $a);
}
