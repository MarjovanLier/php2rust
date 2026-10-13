<?php

declare(strict_types=1);

readonly class Team
{
    /**
     * @param list<int> $scores
     * @param list<string> $members
     */
    public function __construct(public string $name, public array $scores, public array $members, public ?string $coach)
    {
    }

    public function best(): int
    {
        $best = 0;
        foreach ($this->scores as $score) {
            if ($score > $best) {
                $best = $score;
            }
        }
        return $best;
    }

    public function roll(): string
    {
        $out = '';
        foreach ($this->members as $member) {
            $out .= $member;
        }
        return $out;
    }

    public function length(): int
    {
        return strlen($this->name) + count($this->members);
    }
}

function greet(string $name, ?string $coach): string
{
    return $name . ($coach ?? '-');
}

$scores = [3, 9, 4];
$members = ['ann', 'bo'];
$team = new Team('owls', $scores, $members, 'kim');
echo $team->best(), ' ', $team->roll(), ' ', $team->length(), "\n";
echo greet($team->name, $team->coach), "\n";
$teams = [$team];
echo $teams[0]->members[1], ' ', $teams[0]->name, "\n";
$name = $team->name;
echo $name, $team->name, "\n";
