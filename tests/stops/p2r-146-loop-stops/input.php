<?php

declare(strict_types=1);

final class Seat
{
    public int $taken = 0;
}

final class Hall
{
    /** @var list<Seat> */
    public array $seats = [];
    public int $n = 0;

    public function bump(): void
    {
        $this->n++;
    }

    public function readsList(): void
    {
        foreach ($this->seats as $seat) {
            $seat->taken = count($this->seats);
        }
    }

    public function callsRoot(): void
    {
        foreach ($this->seats as $seat) {
            $seat->taken = 1;
            $this->bump();
        }
    }

    public function passesRoot(): void
    {
        foreach ($this->seats as $seat) {
            $seat->taken = 1;
            useHall($this);
        }
    }

    public function extends(): void
    {
        foreach ($this->seats as $seat) {
            $this->seats[0]->taken = 2;
            $seat->taken = 1;
        }
    }

    public function size(): int
    {
        return count($this->seats);
    }

    public function sharedCallWhileChanging(): void
    {
        foreach ($this->seats as $seat) {
            $seat->taken = $this->size();
        }
    }
}

final class Block
{
    /** @var list<Seat> */
    public array $seats = [];
}

final class Venue
{
    public function __construct(public Block $block)
    {
    }

    public function partOfChain(): void
    {
        foreach ($this->block->seats as $seat) {
            $seat->taken = count($this->block->seats) + 1;
            useBlock($this->block);
        }
    }
}

function useBlock(Block $block): void
{
    echo count($block->seats), "\n";
}

function useHall(Hall $hall): void
{
    $hall->n = 0;
}

/**
 * @return list<Seat>
 */
function makeSeats(): array
{
    return [new Seat()];
}

function fromCall(): void
{
    foreach (makeSeats() as $seat) {
        $seat->taken = 1;
    }
}

final class Row
{
    /** @var list<Row> */
    public array $rows = [];
    public int $n = 0;
}

function sameName(Row $x): void
{
    foreach ($x->rows as $x) {
        $x->n = 1;
    }
}
