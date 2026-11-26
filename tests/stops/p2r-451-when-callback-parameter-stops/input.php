<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name', 'rank'];

    public $timestamps = false;
}

// A callback without the query parameter.
/**
 * @return Collection<int, Code>
 */
function noQuery(?string $name): Collection
{
    return Code::query()->when($name, fn () => null)->get();
}

// A query parameter taken by reference.
/**
 * @return Collection<int, Code>
 */
function byReference(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder &$query) => $query->where('name', 'a'))->get();
}

// A query parameter of a type other than Builder, which Mago accepts.
/**
 * @return Collection<int, Code>
 */
function mixedQuery(?string $name): Collection
{
    return Code::query()->when($name, fn (mixed $query) => $query->where('name', 'a'))->get();
}
