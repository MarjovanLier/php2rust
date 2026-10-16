<?php

declare(strict_types=1);

namespace App;

use App\Observers\CodeObserver;
use App\Observers\Watcher;
use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Model;

// Forms of #[ObservedBy] that P2R-460 does not admit, which keep P2R-401's stop: a string,
// a named argument, a class nobody declares, an empty list, another attribute beside it,
// an interface, and a second argument (P2R-462).
#[ObservedBy('App\Observers\CodeObserver')]
final class Str extends Model
{
}

#[ObservedBy(classes: [CodeObserver::class])]
final class Named extends Model
{
}

#[ObservedBy([Missing::class])]
final class Unknown extends Model
{
}

#[ObservedBy([])]
final class Empty1 extends Model
{
}

#[ObservedBy(CodeObserver::class)]
#[\AllowDynamicProperties]
final class Mixed1 extends Model
{
}

#[ObservedBy(Watcher::class)]
final class Iface extends Model
{
}

#[ObservedBy(CodeObserver::class, CodeObserver::class)]
final class Two extends Model
{
}
