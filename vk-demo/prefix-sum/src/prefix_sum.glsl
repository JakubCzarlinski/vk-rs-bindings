#version 460
#extension GL_KHR_shader_subgroup_arithmetic : require
const uint BLOCK_SIZE = 1024;
const uint BLOCK_COUNT = 64;
const uint LOCAL_SIZE = 128;
const uint ITEMS_PER_THREAD = 8;
const uint AGGREGATE_BASE = 0;
const uint PREFIX_BASE = BLOCK_COUNT;
const uint STATUS_BASE = BLOCK_COUNT * 2;
const uint STATUS_INVALID = 0;
const uint STATUS_AGGREGATE = 1;
const uint STATUS_PREFIX = 2;

layout(local_size_x = 128, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer DataBuffer {
  uint values[];
} dataBuffer;

layout(set = 0, binding = 1) coherent buffer LookbackBuffer {
  uint values[];
} lookback;

shared uint subgroupTotals[32];
shared uint subgroupPrefixes[32];
shared uint blockAggregate;
shared uint tilePrefix;

uint scan_tile(uint localSum, out uint threadPrefix)
{
  uint subgroupPrefix = subgroupExclusiveAdd(localSum);
  uint subgroupTotal = subgroupAdd(localSum);

  if (gl_SubgroupInvocationID == gl_SubgroupSize - 1) {
    subgroupTotals[gl_SubgroupID] = subgroupTotal;
  }
  barrier();

  if (gl_SubgroupID == 0) {
    uint value = gl_SubgroupInvocationID < gl_NumSubgroups
      ? subgroupTotals[gl_SubgroupInvocationID] : 0;
    uint prefix = subgroupExclusiveAdd(value);

    if (gl_SubgroupInvocationID < gl_NumSubgroups) {
      subgroupPrefixes[gl_SubgroupInvocationID] = prefix;
    }
    if (gl_SubgroupInvocationID == gl_NumSubgroups - 1) {
      blockAggregate = prefix + value;
    }
  }
  barrier();

  threadPrefix = subgroupPrefixes[gl_SubgroupID] + subgroupPrefix;
  return blockAggregate;
}

uint wait_for_status(uint blockIndex)
{
  uint status = STATUS_INVALID;
  while (status == STATUS_INVALID) {
    status = atomicOr(lookback.values[STATUS_BASE + blockIndex], 0);
  }
  memoryBarrierBuffer();
  return status;
}

void publish_aggregate(uint blockIndex, uint aggregate)
{
  lookback.values[AGGREGATE_BASE + blockIndex] = aggregate;
  memoryBarrierBuffer();
  atomicExchange(lookback.values[STATUS_BASE + blockIndex], STATUS_AGGREGATE);
}

void publish_prefix(uint blockIndex, uint inclusivePrefix)
{
  lookback.values[PREFIX_BASE + blockIndex] = inclusivePrefix;
  memoryBarrierBuffer();
  atomicExchange(lookback.values[STATUS_BASE + blockIndex], STATUS_PREFIX);
}

uint look_back_prefix(uint blockIndex)
{
  if (blockIndex == 0) {
    return 0;
  }

  uint prefix = 0;
  uint cursor = blockIndex - 1;

  for (; ; ) {
    uint status = wait_for_status(cursor);
    if (status == STATUS_PREFIX) {
      prefix += lookback.values[PREFIX_BASE + cursor];
      break;
    }

    prefix += lookback.values[AGGREGATE_BASE + cursor];
    if (cursor == 0) {
      break;
    }
    cursor--;
  }

  return prefix;
}

void main()
{
  uint lid = gl_LocalInvocationID.x;
  uint blockIndex = gl_WorkGroupID.x;
  uint base = blockIndex * BLOCK_SIZE;
  uint itemBase = lid * ITEMS_PER_THREAD;
  uint localPrefix[ITEMS_PER_THREAD];
  uint localSum = 0;

  for (uint i = 0; i < ITEMS_PER_THREAD; i++) {
    uint value = dataBuffer.values[base + itemBase + i];
    localPrefix[i] = localSum;
    localSum += value;
  }

  uint threadPrefix;
  uint aggregate = scan_tile(localSum, threadPrefix);

  if (lid == 0) {
    publish_aggregate(blockIndex, aggregate);

    uint prefix = look_back_prefix(blockIndex);
    tilePrefix = prefix;
    publish_prefix(blockIndex, prefix + aggregate);
  }
  barrier();

  for (uint i = 0; i < ITEMS_PER_THREAD; i++) {
    dataBuffer.values[base + itemBase + i] = localPrefix[i] + threadPrefix + tilePrefix;
  }
}
