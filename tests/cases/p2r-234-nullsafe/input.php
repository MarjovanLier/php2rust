<?php

declare(strict_types=1);

enum Level: int
{
    case Low = 1;
    case High = 2;
}

final readonly class Boss
{
    public function __construct(
        public ?string $nick,
        public string $name,
        public ?Level $level,
    ) {}

    public function label(string $prefix): string
    {
        return $prefix . $this->name;
    }

    public function nickname(): ?string
    {
        return $this->nick;
    }
}

final readonly class Team
{
    public function __construct(public ?Boss $boss) {}

    public function bossName(): ?string
    {
        return $this->boss?->name;
    }
}

function nickOf(?Boss $boss): string
{
    return $boss?->nick ?? 'none';
}

/** @param array{level: ?Level} $s */
function levelOf(array $s): string
{
    return $s['level']?->name ?? 'none';
}

$big = new Boss('bb', 'Big', Level::High);
$team = new Team($big);
$empty = new Team(null);
$full = $team->bossName();
$none = $empty->bossName();
echo $full ?? 'none', ' ', $none ?? 'none', "\n";
echo nickOf($big), ' ', nickOf(null), ' ', nickOf(new Boss(null, 'X', null)), "\n";
$prefix = 'L:';
echo $team->boss?->label($prefix) ?? 'none', ' ', $empty->boss?->label($prefix) ?? 'none', "\n";
echo $team->boss?->nickname() ?? 'none', "\n";
$b = $team->boss;
$level = $b?->level;
echo $level?->value ?? 0, ' ', $level?->name ?? 'none', "\n";
$name = $empty->boss?->name ?? 'nobody';
echo $name, "\n";
echo levelOf(['level' => Level::Low]), ' ', levelOf(['level' => null]), "\n";
