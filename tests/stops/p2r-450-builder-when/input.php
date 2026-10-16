<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

// The builder's own when() runs the callback for a falsy value, so the link stops (P2R-450).
/**
 * @return Collection<int, Code>
 */
function filtered(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder $query, string $name) => $query->where('name', $name))->get();
}
