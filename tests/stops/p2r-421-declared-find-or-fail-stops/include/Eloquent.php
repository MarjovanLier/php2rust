<?php

// A stand-in for Eloquent whose Model declares findOrFail itself. It is not the framework.
// The tool then patches none of create, find and findOrFail, so create stays mixed.

namespace Illuminate\Database\Eloquent;

abstract class Model
{
    public function __get(string $key): mixed
    {
        return null;
    }

    public static function findOrFail(int $id): static
    {
        throw new \LogicException('a stand-in');
    }

    /**
     * @param list<mixed> $arguments
     */
    public static function __callStatic(string $method, array $arguments): mixed
    {
        throw new \LogicException('a stand-in');
    }
}
