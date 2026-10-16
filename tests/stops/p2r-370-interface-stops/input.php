<?php

declare(strict_types=1);

namespace App;

// Constants, static methods and parents wait for a later slice.
interface WithConstant
{
    public const LIMIT = 3;

    public function limit(): int;
}

interface WithStatic
{
    public static function make(): int;
}

interface Base
{
    public function base(): int;
}

interface Child extends Base
{
    public function child(): int;
}

interface Marked
{
    #[\Deprecated]
    public function old(): int;
}

// Parameter forms a trait method does not take as PHP writes them.
interface Defaults
{
    public function padded(int $width = 3): int;
}

interface References
{
    public function changed(int &$count): int;
}

interface Variadic
{
    public function joined(string ...$parts): string;
}

// Return types with no Rust form here.
interface Fluent
{
    public function copy(): static;
}

interface Untyped
{
    public function anything();
}

// PHP calls this one by itself.
interface Showable
{
    public function __toString(): string;
}

// PHP rejects these forms, and Mago's analyser does not report them.
interface Rejected
{
    final public function sealed(): int;

    public function filled(): int
    {
        return 1;
    }

    public function &shared(): int;

    public function getLabel(): string;

    public function GETLABEL(): string;

    abstract public function hollow(): int;

    public public function doubled(): int;

    protected function hidden(): int;

    public function shielded(#[\SensitiveParameter] string $secret): int;
}

// A prelude trait gives every type a method of this name.
interface Convertible
{
    public function into(): string;
}

// PHP reserves this name.
interface Mixed
{
    public function anything(): int;
}

// A class takes the Rust name of this interface.
interface Item_Record
{
    public function id(): int;
}

final readonly class ItemRecord
{
    public function id(): int
    {
        return 1;
    }
}

// A shape struct cannot take the name of a trait.
interface NameSizeShape
{
    public function size(): int;
}

/** @param array{name: string, size: int} $row */
function shaped(array $row): int
{
    return $row['size'];
}

// An interface declared inside another statement.
if (true) {
    interface Late
    {
        public function late(): int;
    }
}
