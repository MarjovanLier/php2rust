<?php

declare(strict_types=1);

enum Size: string
{
    case Small = 's';
    case Large = 'l';

    public static function tryfrom(string $code): ?self
    {
        return null;
    }

    public function name(): string
    {
        return 'size';
    }

    public function fooBar(): int
    {
        return 1;
    }

    public function foobar(): int
    {
        return 2;
    }

    final private function hidden(): int
    {
        return 3;
    }

    public function __construct() {}

    /**
     * @param ?list<Cell> $cells
     */
    public function count(?array $cells): int
    {
        return 0;
    }
}

final class Cell
{
    public int $n = 0;
}

function pick(?Size $size): int
{
    return $size->fooBar();
}

echo pick(Size::Small), "\n";
