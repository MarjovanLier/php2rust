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
