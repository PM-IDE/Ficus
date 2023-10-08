import setuptools


def get_install_reqs():
    with open('requirements.txt', 'r') as f:
        install_reqs = f.read().splitlines()
    return install_reqs


install_reqs = get_install_reqs()

setuptools.setup(
    name='ficus',
    version='1.0.0',
    author='Aero',
    author_email='aerooneq@yandex.ru',
    description='Some Process Mining techniques implementations',
    long_description='Some Process Mining techniques implementations',
    long_description_content_type="text/markdown",
    license='private',
    packages=['ficus',
              'ficus.discovery',
              'ficus.log',
              'ficus.analysis',
              'ficus.analysis.patterns',
              'ficus.analysis.common',
              'ficus.pipelines',
              'ficus.pipelines.analysis',
              'ficus.pipelines.analysis.patterns',
              'ficus.pipelines.serialization',
              'ficus.pipelines.discovery',
              'ficus.pipelines.filtering',
              'ficus.pipelines.mutations',
              'ficus.pipelines.contexts',
              'ficus.pipelines.start',
              'ficus.mutations',
              'ficus.filtering',
              'ficus.grpc_pipelines',
              'ficus.grpc_pipelines.models'],
    install_requires=install_reqs,
)
