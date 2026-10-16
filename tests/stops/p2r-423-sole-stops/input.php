<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;

final class Tag extends Model
{
    public $timestamps = false;
}

final class Keyed extends Model
{
    public $timestamps = false;
}

// Laravel loads only the columns a list names.
function someColumns(): Tag
{
    return Tag::query()->where('label', 'a')->sole(['id']);
}

function firstColumns(): Tag
{
    return Tag::query()->where('label', 'a')->firstOrFail(['id']);
}

function byText(): Tag
{
    return Tag::findOrFail('2');
}

function twoArguments(): Tag
{
    return Tag::query()->findOrFail(2, ['id']);
}

function named(): Tag
{
    return Tag::findOrFail(id: 2);
}

// sole and firstOrFail follow one where or more.
function everyRow(): Tag
{
    return Tag::query()->sole();
}

// A where on the model class goes through __callStatic, which Mago types as mixed.
function staticWhere(): Tag
{
    return Tag::where('label', 'a')->firstOrFail();
}

function filteredFind(): Tag
{
    return Tag::query()->where('label', 'a')->findOrFail(2);
}

// The store of Keyed does not apply (P2R-410).
function notNumbered(): Keyed
{
    return Keyed::findOrFail(1);
}

// A call that throws cannot stand in the default of ??, which Rust runs in a closure.
function either(?Tag $tag): Tag
{
    return $tag ?? Tag::findOrFail(1);
}

function thrown(): void
{
    throw new ModelNotFoundException('none');
}
