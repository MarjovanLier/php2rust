<?php

declare(strict_types=1);

enum Mood
{
    case Calm;
    case Busy;
}

final class Settings
{
    public int $retries = 3;
    public int $offset = -2;
    public float $ratio = 2;
    public float $limit = -0.5;
    public bool $verbose = false;
    public string $label = "main";
    public ?string $note = null;
    public ?int $cap = 10;
    /** @var list<string> */
    public array $tags = [];
    public Mood $mood = Mood::Calm;

    public function __construct(public readonly string $name)
    {
    }
}

class Flag
{
    public bool $on = true;
}

$s = new Settings("box");
$s->tags[] = "x";
echo $s->name, " ", $s->retries + $s->offset, " ", $s->label, " ", count($s->tags), "\n";
if ($s->ratio > $s->limit && !$s->verbose && $s->mood === Mood::Calm) {
    echo "calm\n";
}
$f = new Flag();
if ($f->on) {
    echo "on\n";
}
