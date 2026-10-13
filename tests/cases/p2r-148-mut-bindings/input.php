<?php

declare(strict_types=1);

final class Lamp
{
    public bool $lit = false;

    public function toggle(): void
    {
        $this->lit = !$this->lit;
    }
}

final class Room
{
    /** @var list<Lamp> */
    public array $lamps = [];
}

function lightUp(Lamp $lamp): Room
{
    $lamp->toggle();
    $room = new Room();
    $room->lamps[] = $lamp;
    return $room;
}

$read = new Lamp();
$changed = new Lamp();
$changed->toggle();
$room = lightUp($changed);
foreach ($room->lamps as $lamp) {
    $lamp->toggle();
}
if (!$read->lit && $room->lamps[0]->lit) {
    echo "one lit\n";
}
