<?php

declare(strict_types=1);

final class Greeter
{
    public function hello(): string
    {
        return "hello";
    }

    public static function make(): string
    {
        return "made";
    }
}

function maybe(): ?Greeter
{
    return new Greeter();
}

$greeter = new Greeter();
$name = "hello";
echo $greeter->$name(), "\n";

$factory = "make";
echo Greeter::$factory(), "\n";

$optional = maybe();
$method = "hello";
echo $optional?->$method() ?? "none", "\n";
