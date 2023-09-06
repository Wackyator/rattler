from rattler.rattler import PyAuthenticatedClient, PyFetchRepoDataOptions, fetch_repo_data
import asyncio

async def func(subdir, client, cache, options):
    return await fetch_repo_data(subdir, client, cache, options)


if __name__ == "__main__":
    subdir = "https://conda.anaconda.org/conda-forge/linux-64"
    client = PyAuthenticatedClient()
    cache = "/home/toaster/code/prefix/rattler/py-rattler/cache/repodata"
    options = PyFetchRepoDataOptions()
    
    data = asyncio.run(func(subdir, client, cache, options))
    print(f'type(data) = {type(data).__name__}\n')
    print(data.as_str())
