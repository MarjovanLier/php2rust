<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;

final class Tag extends Model
{
    protected $fillable = ['name', 'code', 'size'];

    public $timestamps = false;
}

final class Locked extends Model
{
    public $timestamps = false;
}

final class Partial extends Model
{
    protected $fillable = ['name', 'code'];

    public $timestamps = false;
}

final class Dated extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

final class Keyed extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

// Laravel writes created_at and updated_at, which this table does not have.
final class Stamped extends Model
{
    protected $fillable = ['name'];
}

final class Open extends Model
{
    protected $guarded = [];

    public $timestamps = false;
}

final class Amount extends Model
{
    protected $fillable = ['amount'];

    protected $casts = ['amount' => 'string'];

    public $timestamps = false;
}

// Laravel reads the values of $fillable, so a keyed list stops.
final class Mapped extends Model
{
    protected $fillable = ['a' => 'name'];

    public $timestamps = false;
}

// SQLite stores a double as REAL, which rounds an int above 2^53.
final class Measure extends Model
{
    protected $fillable = ['size'];

    protected $casts = ['size' => 'integer'];

    public $timestamps = false;
}

// useCurrent() gives a default only to a date or time column.
final class Flagged extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

final class Seeded extends Model
{
}

final class Soft extends Model
{
    use SoftDeletes;
}

final class Left extends Model
{
    protected $table = 'pairs';
}

final class Right extends Model
{
    protected $table = 'pairs';
}

// create stops on each of these (P2R-411).
function withDefault(): Tag
{
    return Tag::create(['name' => 'a', 'size' => 1]);
}

function withId(): Tag
{
    return Tag::create(['id' => 3, 'name' => 'a', 'code' => 'b', 'size' => 1]);
}

function guarded(): Locked
{
    return Locked::create(['name' => 'a']);
}

function wrongType(): Tag
{
    return Tag::create(['name' => 'a', 'code' => 'b', 'size' => '1']);
}

function missing(): Tag
{
    return Tag::create(['name' => 'a', 'code' => 'b']);
}

function leftOut(): Dated
{
    return Dated::create(['name' => 'a']);
}

function stamped(): Stamped
{
    return Stamped::create(['name' => 'a']);
}

function flagged(): Flagged
{
    return Flagged::create(['name' => 'a']);
}

function mapped(): Mapped
{
    return Mapped::create(['name' => 'a']);
}

function realInt(): Measure
{
    return Measure::create(['size' => 9007199254740993]);
}

function twice(): Tag
{
    return Tag::create(['name' => 'a', 'name' => 'b', 'code' => 'c', 'size' => 1]);
}

function notLiteral(): Tag
{
    return Tag::create(['name' => 'a'] + ['code' => 'b', 'size' => 1]);
}

function notFillable(): Partial
{
    return Partial::create(['name' => 'a', 'code' => 'b', 'size' => 1]);
}

function underscored(): Open
{
    return Open::create(['name' => 'a', '_hash' => 'h']);
}

function numericText(): Amount
{
    return Amount::create(['amount' => '1.0']);
}

// Every form stops on these models (P2R-410).
function notNumbered(): ?Keyed
{
    return Keyed::query()->find(1);
}

function shared(): ?Left
{
    return Left::query()->find(1);
}

function seeded(): ?Seeded
{
    return Seeded::query()->find(1);
}

function soft(): ?Soft
{
    return Soft::query()->find(1);
}

// find and where stop on these (P2R-412, P2R-413).
function byText(string $id): ?Tag
{
    return Tag::query()->find($id);
}

function unknownColumn(): ?Tag
{
    return Tag::query()->where('nothing', 'a')->first();
}

function changing(string $name): ?Tag
{
    return Tag::query()->where('name', strtoupper($name))->first();
}

function maybe(?string $name): ?Tag
{
    return Tag::query()->where('name', $name)->first();
}

function nullForPlain(): ?Tag
{
    return Tag::query()->where('name', null)->first();
}

function numericWhere(): ?Amount
{
    return Amount::query()->where('amount', '1')->first();
}

function realWhere(): ?Measure
{
    return Measure::query()->where('size', 9007199254740993)->first();
}

function named(string $name): ?Tag
{
    return Tag::query()->where('name', value: $name)->first();
}

// Any other call on a model or its query stops (P2R-414).
function held(): ?Tag
{
    $query = Tag::query();
    return $query->first();
}

function plain(): ?Tag
{
    return Tag::query()->first();
}

function someColumns(): ?Tag
{
    return Tag::query()->where('name', 'a')->first(['id']);
}
