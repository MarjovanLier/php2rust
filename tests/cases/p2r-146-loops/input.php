<?php

declare(strict_types=1);

final class Seat
{
    public int $taken = 0;

    public function empty(): bool
    {
        return $this->taken === 0;
    }
}

final class Hall
{
    /** @var list<Seat> */
    public array $seats = [];
    public int $booked = 0;
    /** @var list<int> */
    public array $log = [];

    public function book(): void
    {
        foreach ($this->seats as $seat) {
            $seat->taken = 1;
            $this->booked++;
        }
    }

    public function count(): int
    {
        $n = 0;
        foreach ($this->seats as $i => $seat) {
            if (!$seat->empty() && $this->open()) {
                $n += $i;
            }
        }
        return $n;
    }

    public function open(): bool
    {
        return $this->booked < 10;
    }

    public function replay(): void
    {
        foreach ($this->log as $entry) {
            $this->booked += $entry;
        }
    }
}

/**
 * @param list<Seat> $seats
 */
function clear(array $seats): void
{
    foreach ($seats as $k => $seat) {
        $seat->taken = $k;
    }
}

/**
 * @param list<Seat> $seats
 */
function mark(array $seats): int
{
    $n = 0;
    foreach ($seats as $seat) {
        $n += $seat->taken;
    }
    $seats[0]->taken = 9;
    return $n;
}

$hall = new Hall();
$hall->seats[] = new Seat();
$hall->seats[] = new Seat();
$hall->log[] = 2;
$hall->book();
$hall->replay();
echo $hall->booked, " ", $hall->count(), "\n";
clear($hall->seats);
echo mark($hall->seats), "\n";
echo $hall->seats[0]->taken, "\n";
$list = [new Seat(), new Seat()];
foreach ($list as $seat) {
    $seat->taken = 3;
}
foreach ($list as $seat) {
    echo $seat->taken;
}
echo "\n";
