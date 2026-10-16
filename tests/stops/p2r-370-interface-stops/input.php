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
}

// A prelude trait gives every type a method of this name.
interface Convertible
{
    public function into(): string;
}

// A trait of Rust's prelude takes this name.
interface ToString
{
    public function text(): string;
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
